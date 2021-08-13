use derive_more::{Display, Error};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Error, Display, Debug)]
pub enum PostError {
    #[display(fmt = "Post exceeds the character limit")]
    TooLong,
    #[display(fmt = "post is empty or contains only whitespace")]
    EmptyOrWhitespace,
    #[display(fmt = "Invalid post")]
    InvalidPost,
}

#[derive(Debug)]
pub struct PostBody(String);

impl PostBody {
    pub fn parse(body: String) -> Result<PostBody, PostError> {
        let too_long = body.graphemes(true).count() > 10000;
        let whitespace_or_empty = body.trim().is_empty();

        if too_long || whitespace_or_empty {
            match too_long {
                true => Err(PostError::TooLong),
                _ => match whitespace_or_empty {
                    true => Err(PostError::EmptyOrWhitespace),
                    _ => Err(PostError::InvalidPost),
                },
            }
        } else {
            Ok(PostBody(body))
        }
    }
}

impl AsRef<str> for PostBody {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
