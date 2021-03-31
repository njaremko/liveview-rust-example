#[derive(askama::Template, Clone, Debug, Default)]
#[template(path = "hello.html")]
pub struct ExTemplate {
    pub name: String,
    pub count: i32,
}

impl live_view::Template for ExTemplate {
    fn render(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(<Self as askama::Template>::render(&self)?)
    }
}
