#![recursion_limit="2048"]
#[macro_use]
extern crate yew;
extern crate stdweb;
extern crate regex;
extern crate svg;

pub mod api;
pub mod api_yew;
pub mod parsing;
pub mod build_class_svg;
pub mod build_usecase_svg;
pub mod svglib;
pub mod get_diagram_type;
pub mod examples;

fn main() {
    let api = api::build_api(api::Eingaben::WEBTEXT, api::Ausgaben::SVGWEB);
    api.start();
}
