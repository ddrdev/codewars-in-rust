pub fn hello(name: &str) -> String {
    if name.trim().is_empty() {
        String::from("Hello, World!")
    } else {
        format!(
            "Hello, {}{}!",
            name[..1].to_uppercase(),
            name[1..].to_lowercase()
        )
    }
}
