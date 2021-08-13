#[derive(Debug)]
pub struct Image {
    pub path: String,
}

impl Image {
    pub fn new(path: String) -> Result<Image, String> {
        Ok(Image { path: path })
    }
}

impl AsRef<str> for Image {
    fn as_ref(&self) -> &str {
        &self.path
    }
}
