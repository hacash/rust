
fn json_path(a: &String, b: &str) -> Box<Path> {
    Path::new(a).join(b). into_boxed_path()
}