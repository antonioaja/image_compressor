pub mod jpg;
pub mod png;

pub fn ext_is_valid(filename: &str) -> bool {
    matches!(
        extension(filename),
        ".jpg" | ".jpeg" | ".png" | ".jxl" | ".avif" | ".gif" | ".bmp"
    )
}
pub fn extension(filename: &str) -> &str {
    filename
        .rfind('.')
        .map(|idx| &filename[idx..])
        .filter(|ext| ext.chars().skip(1).all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("")
}
