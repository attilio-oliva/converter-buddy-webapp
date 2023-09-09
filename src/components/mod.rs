mod export_form;
mod import_form;

pub use export_form::*;
pub use import_form::*;

use converter_buddy::{config::Config, converter::Converter, format::Format};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::go_icons::{GoInfo, GoMarkGithub},
    Icon,
};
use strum::IntoEnumIterator;

type FileList = Vec<(String, Vec<u8>)>;

pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, FileList::new);
    use_shared_state_provider(cx, || Option::<Config>::None);

    cx.render(rsx! {
        header {
            Logo {}
            Menu {}
        }
        div { class: "content-container",
            ImportForm {}
            ExportForm {}
        }
        InfoPopup {}
    })
}

pub fn Logo(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "navbar-left",
            a { href: "/", class: "logo", img { src: "images/logo.svg", alt: "Converter Buddy" } }
        }
    })
}

pub fn Menu(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "navbar-right",
            a { href: "/", class: "navbar-item",
                label { r#for: "compatibility-popup", class: "navbar-item open-popup-btn",
                    Icon { icon: GoInfo, width: 24, height: 24 }
                }
            }
            a {
                href: "https://github.com/attilio-oliva/converter-buddy-webapp",
                class: "navbar-item",
                Icon { icon: GoMarkGithub, width: 24, height: 24 }
            }
        }
    })
}

pub fn InfoPopup(cx: Scope) -> Element {
    cx.render(rsx! {
        input { class: "modal-state", id: "compatibility-popup", r#type: "checkbox" }
        div { class: "modal",
            label { class: "modal__bg", r#for: "compatibility-popup" }
            div { class: "modal__inner",
                label { class: "modal__close", r#for: "compatibility-popup" }
                h2 { "What is this website?" }
                ProjectInfo {}
                h3 { "Currently supported formats" }
                SupportedFormatsInfo {}
            }
        }
    })
}

pub fn ProjectInfo(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            b { "Converter Buddy" }
            " is a file conversion utility. This website uses Converter Buddy in your "
            b { "local browser" }
            " rather than a server, so that your "
            b { "data is not shared online" }
            ". The whole project is "
            b { "open-source" }
            " with a MIT license and can be found on "
            a { href: "https://github.com/attilio-oliva/converter-buddy-webapp", "GitHub" }
        }
    })
}

pub fn SupportedFormatsInfo(cx: Scope) -> Element {
    let supported_list = Format::iter()
        .filter_map(|source_format| {
            // if exists a converter with this format as input
            if let Ok(converter) = Converter::try_from(source_format) {
                return Some((source_format, converter.supported_formats()));
            }
            None
        })
        .collect::<Vec<(Format, Vec<Format>)>>();

    cx.render(rsx! {
        ul {
            for (source_format , supported_formats) in supported_list.iter() {
                li { format!("{} → {:?}", source_format, supported_formats) }
            }
        }
    })
}
