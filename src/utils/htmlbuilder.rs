pub struct HtmlBuilder {
    result_html : String
}

impl HtmlBuilder {

    pub fn build(self) -> String {
        self.result_html
    }

    pub fn new() -> Self {
        HtmlBuilder { result_html: String::default() }
    }

    pub fn h1(mut self, text : &str) -> Self {
        self.result_html += format!("<h1>{}</h1>", text).as_str();
        self
    }

    pub fn h2(mut self, text : &str) -> Self {
        self.result_html += format!("<h2>{}</h2>", text).as_str();
        self
    }

    pub fn h3(mut self, text : &str) -> Self {
        self.result_html += format!("<h3>{}</h3>", text).as_str();
        self
    }

    pub fn h4(mut self, text : &str) -> Self {
        self.result_html += format!("<h4>{}</h4>", text).as_str();
        self
    }

    pub fn h5(mut self, text : &str) -> Self {
        self.result_html += format!("<h5>{}</h5>", text).as_str();
        self
    }

    pub fn h6(mut self, text : &str) -> Self {
        self.result_html += format!("<h6>{}</h6>", text).as_str();
        self
    }

    pub fn a(mut self, href : &str, inner_text : &str) -> Self {
        self.result_html += format!("<a href='{}'>{}</a>", href, inner_text).as_str();
        self
    }

    pub fn p(mut self, text : &str) -> Self {
        self.result_html += format!("<p>{}</a>", text).as_str();
        self
    }
}