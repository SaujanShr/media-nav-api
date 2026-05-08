pub fn filename_from_url(url: &str) -> String {
    url.rsplit('/').next().unwrap_or(url).to_string()
}
