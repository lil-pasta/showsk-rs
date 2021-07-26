use unicode_segmentation::UnicodeSegmentation;
use deriv_more::{Display, Error};


#[derive(Debug)]
pub struct PostBody(String);

impl PostBody {
    pub fn parse(body: String) -> Result<PostBody, String> {
        let too_long = body.graphemes(true).count() > 10000;
        let whitespace_or_empty = body.trim().is_empty();

        if too_long || whitespace_or_empty {
            
    }
}
