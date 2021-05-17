use rocket::response::content::Html;
use crate::utils::htmlbuilder::HtmlBuilder;

#[get("/")]
pub fn get() -> Html<String> {
    Html(format!(
        include_str!("../../static/index.html"),
        HtmlBuilder::new().h1("Rusty Snake Multiplayer Matchmaking Home Page").a("/", "Host Room").build())
    )
}
