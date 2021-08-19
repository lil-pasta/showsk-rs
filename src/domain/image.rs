#[derive(Debug)]
pub struct Image {
    pub path: String,
}

impl Image {
    pub fn new(impath: String) -> Result<Image, String> {
        if impath.trim().is_empty() {
            Ok(Image {
                path: "".to_string(),
            })
        } else {
            Ok(Image { path: impath })
        }
    }
}

impl AsRef<str> for Image {
    fn as_ref(&self) -> &str {
        &self.path
    }
}
