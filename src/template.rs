#[derive(askama::Template, Clone, Debug, Default)]
#[template(path = "app.html", escape = "none")]
pub struct AppTemplate {
    pub body: String
}

impl live_view::Template for AppTemplate {
    fn render(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(<Self as askama::Template>::render(&self)?)
    }
}

#[derive(askama::Template, Clone, Debug, Default)]
#[template(path = "hello.html")]
pub struct ExTemplate {
    pub name: String,
    pub count: i32,
    pub touching_text: String,
}

impl live_view::Template for ExTemplate {
    fn render(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(<Self as askama::Template>::render(&self)?)
    }
}
