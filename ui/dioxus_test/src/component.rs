#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
  cx.render(rsx!{
    div {
      "Hello world!"
    }
  })
}