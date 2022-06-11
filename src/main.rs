mod ui;

use dioxus::desktop::tao::dpi::LogicalSize;

fn main() {
    let props = ui::AppProps {
        // margin: "0",
        // padding: "0",
    };
    dioxus::desktop::launch_with_props(ui::app, props, |cfg| {
        cfg.with_window(|w| {
            w.with_title("S3 File Manager")
                .with_inner_size(LogicalSize::new(600.0, 400.0))
        })
        .with_custom_head(format!("<style>{}</style>", include_str!("style.css")))
        //.with_disable_context_menu(true)
    });
}

