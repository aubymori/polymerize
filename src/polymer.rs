use askama::Template;
use crate::polymer;

pub const REVISION: &str = "18069be1";
pub const JS_NONCE: &str = "tZ6lf2cgpM1Lr7NaCwIpuw";
pub const CSS_NONCE: &str = "4wlc/lIBt+6cn8bXq517jA";

#[derive(Template)]
#[template(path = "core.html")]
pub struct CoreTemplate<'a> {
    pub config: &'a str,
    pub initial_data: &'a str,
}