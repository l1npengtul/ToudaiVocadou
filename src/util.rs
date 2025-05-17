pub fn shorten(content: &str) -> String {
    content.chars().take(150).collect::<String>()
}
