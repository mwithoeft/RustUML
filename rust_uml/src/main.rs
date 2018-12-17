#![recursion_limit="1024"]
extern crate yew;
extern crate stdweb;
extern crate rust_uml;

extern crate regex;
extern crate svg;

use yew::prelude::*;
use stdweb::web::{IElement, INode, IParentNode, document};
use rust_uml::Model;

fn main() {
    yew::initialize();

    //Für Docsify
    //let body = document().query_selector("article").unwrap().unwrap();


    let body = document().query_selector("body").unwrap().unwrap();


    let mount_class = "rust_uml";
    let mount_point = document().create_element("div").unwrap();
    mount_point.class_list().add(mount_class).unwrap();
    body.append_child(&mount_point);

    App::<Model>::new().mount(mount_point);
    yew::run_loop();
}
