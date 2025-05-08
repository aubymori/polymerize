#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;
use askama::Template;
mod polymer;

#[get("/")]
fn home() -> RawHtml<String> {
    let tmpl = polymer::CoreTemplate { config: "{\"test\":\"test\"}", initial_data: "shit" };
    RawHtml(tmpl.render().unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/", routes![home]
    )
}