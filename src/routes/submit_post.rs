use crate::domain::post::NewPost;
use crate::startup::AppData;
use actix_multipart::Multipart;
use actix_web::{error, http::StatusCode, post, web, HttpResponse, HttpResponseBuilder, Result};
use chrono::Utc;
use derive_more::{Display, Error};
use futures::{StreamExt, TryStreamExt};
use sqlx::PgPool;
use std::fs;
use std::io::Write;
use uuid::Uuid;

// custom error handler for the route
#[derive(Debug, Display, Error)]
pub enum NewPostError {
    #[display(fmt = "An internal error occured. Please try again later")]
    QueryError,
    #[display(fmt = "Error uploading your file")]
    FileUploadError,
    #[display(fmt = "Error parsing submitted fields")]
    ParseError,
}

impl error::ResponseError for NewPostError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            NewPostError::QueryError => StatusCode::INTERNAL_SERVER_ERROR,
            NewPostError::FileUploadError => StatusCode::INTERNAL_SERVER_ERROR,
            NewPostError::ParseError => StatusCode::BAD_REQUEST,
        }
    }
}

#[post("/submit_post")]
#[tracing::instrument(name = "adding a new post", skip(payload, data))]
pub async fn submit_post(
    payload: Multipart,
    data: web::Data<AppData>,
) -> Result<HttpResponse, NewPostError> {
    // use your domain! now there is only a single access point
    // for the api which should greatly increase app security and reliability
    let new_post = build_post(payload, &data.upload_path).await?;
    insert_post(&new_post, &data.db_pool)
        .await
        .map_err(|_| NewPostError::QueryError)?;
    Ok(HttpResponse::Ok().finish())
}

// Take the payload from a multipart/form-data post submission and turn it into
// a valid post
// TODO: allow for multiple image uploads?
#[tracing::instrument(name = "parsing post submission", skip(payload))]
pub async fn build_post(mut payload: Multipart, u_path: &str) -> Result<NewPost, NewPostError> {
    // prep upload dest and create our text payload
    fs::create_dir_all(&u_path).map_err(|_| NewPostError::FileUploadError)?;
    let mut text_body = Vec::new();
    let mut filename = "".to_string();
    let mut filepath = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        // check disposition for field name
        // NOTE: this could be made more general/ergonomic
        if content_type.get_name().unwrap() == "post-editor" {
            // have to iterate over our text body byte stream
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                let body_str =
                    String::from_utf8(data.to_vec()).map_err(|_| NewPostError::ParseError)?;
                text_body.push(body_str);
            }
        }
        // same as above
        else if content_type.get_name().unwrap() == "image" {
            let filename = format!(
                "{}-{}",
                Uuid::new_v4(),
                content_type.get_filename().unwrap()
            );
            let filepath = format!("{}/{}", u_path, sanitize_filename::sanitize(&filename));
            let mut f = web::block(move || std::fs::File::create(filepath.clone()))
                .await
                .map_err(|_| NewPostError::FileUploadError)?
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .map_err(|_| NewPostError::FileUploadError)?
                    .unwrap();
            }
        }
    }
    let body = text_body.join(" ");
    let new_post = NewPost::new(body, filename, filepath).map_err(|_| NewPostError::ParseError);
    new_post
}

// send the post to the db.
// TODO: add user_id once you've figured out session data
#[tracing::instrument(name = "performing new user insert", skip(db_pool))]
pub async fn insert_post(post: &NewPost, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO post (post_id, body, image, timestmp)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        post.body.as_ref(),
        post.image.path,
        Utc::now(),
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert query: {:?}", e);
        e
    })?;
    Ok(())
}