use sanitize_filename;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Image {
    name: ImageName,
    pub path: String,
}

impl Image {
    pub fn new(name: String, path: String) -> Result<Image, String> {
        let iname = ImageName::parse(name)?;
        Ok(Image {
            name: iname,
            path: path,
        })
    }
}

#[derive(Debug)]
pub struct ImageName(String);

impl ImageName {
    pub fn parse(mut iname: String) -> Result<ImageName, String> {
        iname = sanitize_filename::sanitize(iname);

        let is_empty_or_whitespace = iname.trim().is_empty();

        let too_long = iname.graphemes(true).count() > 256;

        if too_long {
            Err(format!("image name is invalid: {}", iname))
        } else if is_empty_or_whitespace {
            iname = "".to_string();
            Ok(Self(iname))
        } else {
            Ok(Self(iname))
        }
    }
}

impl AsRef<str> for ImageName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
