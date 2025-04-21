pub fn wrapped_text(text: &str, is_simple: bool) -> String {
    if is_simple { text.to_string() }
    else { format!("({})", text) }
}