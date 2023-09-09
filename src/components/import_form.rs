use dioxus::prelude::*;

use super::FileList;

pub fn ImportForm(cx: Scope) -> Element {
    let files_uploaded = use_shared_state::<FileList>(cx).unwrap();
    cx.render(rsx! {
        div { class: "content",
            div {
                prevent_default: "ondrop ondragover",
                ondragover: move |event: DragEvent| {
                    event.stop_propagation();
                },
                ondrop: move |_event: DragEvent| {},
                h2 { class: "content-header", "Select or drop files to convert" }
                div { class: "content-inner-box",
                    if files_uploaded.read().is_empty() {
                        rsx!{p { "No file selected" }}
                    } else {
                        rsx!{p { "Selected files:"},
                             ul {
                                for (filename, _data) in files_uploaded.read().iter() {
                                   li{ "{filename}" } 
                                }
                         }
                        }
                    }
                }
                div { class: "content-inner-box",
                    input {
                        id: "file-button",
                        r#type: "file",
                        multiple: true,
                        hidden: true,
                        onchange: move |event| {
                            to_owned![files_uploaded];
                            async move {
                                if let Some(file_engine) = &event.files {
                                    let files = file_engine.files();
                                    files_uploaded.write_silent().clear();
                                    for file_name in &files {
                                        if let Some(file) = file_engine.read_file(file_name).await {
                                            files_uploaded.write().push((String::from(file_name), file));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    input {
                        r#type: "button",
                        onclick: move |_| { click_file_button() },
                        value: "Select files"
                    }
                }
            }
        }
    })
}

/// This function is used to simulate a click on the hidden button used to open the file dialog.
/// This is used to override the default style of the file input button.
fn click_file_button() {
    let window = web_sys::window().expect("Window should exist");
    let document = window.document().expect("Document should exist");
    let element = document.query_selector("#file-button").unwrap().unwrap();
    let event = document
        .create_event("MouseEvents")
        .expect("should be able to call createEvent()");
    event.init_event_with_bubbles_and_cancelable("click", true, true);
    let _ = element.dispatch_event(&event);
}
