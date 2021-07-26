// use crate::domain::image_path::ImagePath;
use crate::domain::text_body::PostBody;

#[derive(Debug)]
pub struct NewPost {
    body: PostBody,
    //    img_path: ImagePath,
}

impl NewPost {
    pub fn new(body: String) -> Result<NewPost, String> {
        let post_body = PostBody::parse(body)?;

        Ok(NewPost { body: post_body })
    }
}
