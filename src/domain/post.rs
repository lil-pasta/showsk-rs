use crate::domain::image::Image;
use crate::domain::text_body::PostBody;

#[derive(Debug)]
pub struct NewPost {
    pub body: PostBody,
    pub image: Image,
}

impl NewPost {
    pub fn new(body: String, name: String, path: String) -> Result<NewPost, String> {
        let post_body = PostBody::parse(body).map_err(|e| e.to_string())?;
        let img = Image::new(name, path).map_err(|e| e.to_string())?;
        Ok(NewPost {
            body: post_body,
            image: img,
        })
    }
}
