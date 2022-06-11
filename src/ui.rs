use dioxus::prelude::*;

pub struct AppProps {
}

pub fn app(cx: Scope<AppProps>) -> Element {
    cx.render(rsx! ( 
        div {
                width: "220px",
                background_color: "#ddd",
        },
        div {
            flex_grow: "1",
            background_color: "#eee",
        },
    ))
}