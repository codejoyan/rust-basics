pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if (x.len() < y.len()) {
        y
    } else {
        x
    }
}
