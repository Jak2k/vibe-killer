pub fn get(name: &str) -> String {
    match name {
        "LICENSES" => "yaoi texts with ASCII art",
        "examples" => "examples for bombs and instructions how to build them",
        "src" => "source for our darknet marketplace",
        "target" => "output of our marketplace's build system",
        "templates" => "the HTML templates for the femboy auction system",
        _ => "",
    }
    .to_owned()
}
