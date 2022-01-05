use crate::domain::post::NewPost;
use crate::startup::AppData;
use actix_multipart::Multipart;
use actix_web::{
    error, http, http::StatusCode, post, web, HttpResponse, HttpResponseBuilder, Result,
};
use chrono::Utc;
use derive_more::{Display, Error};
use futures::{StreamExt, TryStreamExt};
use sqlx::PgPool;
use std::fs;
use std::io::Write;
use uuid::Uuid;

// custom error handler for the route
// TODO: switch to a better error writing framework (rather than roll your own)
#[derive(Debug, Display, Error)]
pub enum NewPostError {
    #[display(fmt = "An internal error occured. Please try again later")]
    QueryError,
    #[display(fmt = "Error uploading your file")]
    FileUploadError,
    #[display(fmt = "File upload path error")]
    FileUploadPathError,
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
            NewPostError::FileUploadPathError => StatusCode::INTERNAL_SERVER_ERROR,
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
    // all done redirect to index
    Ok(HttpResponse::Found()
        .append_header((http::header::LOCATION, "/"))
        .finish())
}

// Take the payload from a multipart/form-data post submission and turn it into
// a valid post
// TODO: allow for multiple image uploads?
#[tracing::instrument(name = "parsing post submission", skip(payload))]
pub async fn build_post(mut payload: Multipart, u_path: &str) -> Result<NewPost, NewPostError> {
    // prep upload dest and create our text payload
    let uppath = std::env::current_dir().unwrap().join(&u_path);
    if !std::path::Path::new(&uppath).is_dir() {
        std::fs::create_dir_all(&uppath.to_str().unwrap())
            .map_err(|_| NewPostError::FileUploadPathError)?;
    }
    fs::create_dir_all(&uppath.to_str().unwrap()).map_err(|_| NewPostError::FileUploadError)?;
    let mut text_body = Vec::new();
    let mut filepath = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        // check disposition for field name
        // TODO: more dynamic condition checking
        if content_type.get_name().unwrap() == "post-editor" {
            // have to iterate over our text body byte stream
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                let body_str =
                    String::from_utf8(data.to_vec()).map_err(|_| NewPostError::ParseError)?;
                text_body.push(body_str);
            }
        }
        // same as above but for the other field
        else if content_type.get_name().unwrap() == "image"
            && !content_type.get_filename().unwrap().trim().is_empty()
        {
            let filename = format!(
                "{}-{}",
                Uuid::new_v4(),
                sanitize_filename::sanitize(content_type.get_filename().unwrap())
            );

            // absolute path
            let upload_str = format!("{}/{}", uppath.to_str().unwrap(), filename);
            // relative filepath
            filepath = format!("../{}/{}", u_path, filename);

            let mut f = web::block(move || std::fs::File::create(upload_str))
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
    NewPost::new(body, filepath).map_err(|_| NewPostError::ParseError)
}

// send the post to the db.
// TODO: add user_id once you've figured out session data
#[tracing::instrument(name = "performing new post insert", skip(db_pool, post))]
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
