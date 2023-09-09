use std::{collections::BTreeMap, path::PathBuf, str::FromStr};

use super::FileList;
use converter_buddy::{
    config::Config,
    converter::Converter,
    format::{self, from_extension, Format},
};
use dioxus::prelude::*;

pub fn ExportForm(cx: Scope) -> Element {
    let files_uploaded = use_shared_state::<FileList>(cx).unwrap();
    let config = use_shared_state::<Option<Config>>(cx).unwrap();

    let mut source_format = None;
    if files_uploaded.read().len() > 0 {
        source_format = files_uploaded
            .read()
            .get(0)
            .and_then(|(name, _)| guess_source_format(name.clone()));
    }

    let mut categorized_formats = BTreeMap::new();
    if let Some(guessed_source_format) = source_format {
        if let Ok(converter) = Converter::try_from(guessed_source_format) {
            categorized_formats = create_category_map(&converter.supported_formats());
        }
    }

    cx.render(rsx! {
        div { class: "content",
            h2 { class: "content-header",
                "Desired format"
                div { class: "content-inner-box",
                    select { onchange: move |event| {
                            let target = &event.value;
                            let target_format = match target.as_str() {
                                "" => None,
                                _ => Some(Format::from_str(target).unwrap()),
                            };
                            if let Some(format) = target_format {
                                *config.write() = Some(Config::try_from(format).unwrap());
                            }
                        },
                        option { value: "", "Select a format" }
                        categorized_formats.iter().map(|(category, category_list)| {
                            rsx! {
                            optgroup {label:"{category}",
                            category_list.iter().map(|format| {
                                   rsx! {
                                          option {value: "{format}", "{format}"}
                                   }
                               })
                           }
                       }
                   })
                    }
                }
                div { class: "content-inner-box",
                    input {
                        r#type: "button",
                        value: "Convert",
                        onclick: move |_event| {
                            let Some(config) = config.read().clone() else {
                                panic!("Export configuration not found")
                            };
                            for (name, data) in files_uploaded.read().iter() {
                                convert_and_download(name, data, &config);
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn guess_source_format(file_name: String) -> Option<Format> {
    let ext = file_name.split('.').last();
    ext.and_then(format::from_extension)
}

#[allow(dead_code)]
pub fn supported_formats(source_format: &Option<Format>) -> Vec<Format> {
    let Some(guessed_source_format) = source_format else {
        return vec![];
    };
    let Ok(converter) = Converter::try_from(*guessed_source_format) else {
        return vec![];
    };

    converter.supported_formats()
}

fn create_category_map(formats: &[Format]) -> BTreeMap<String, Vec<Format>> {
    let mut map = BTreeMap::new();
    for format in formats {
        let category = format.kind().to_string();
        map.entry(category).or_insert(vec![]).push(*format);
    }
    map
}

fn convert_and_download(file_name: &str, data: &Vec<u8>, config: &Config) {
    let mut output = Vec::<u8>::new();
    let source_path = PathBuf::from(file_name.to_string());
    let source_ext = source_path.extension().unwrap().to_str().unwrap();

    let target_format: Format = config.clone().into();
    let target_format_ext = target_format.info().preferred_extension;
    let target_path = PathBuf::from(file_name.to_string()).with_extension(target_format_ext);
    let target_file_name = target_path.as_os_str().to_str().unwrap();

    let Some(source_format) = from_extension(source_ext) else {
        panic!("Source format not found")
    };

    let converter = Converter::try_from(source_format).unwrap();
    converter
        .process(data, &mut output, config.clone())
        .expect("Conversion failed");
    download_file_with_uri(
        bytes_to_base64_uri(&output, target_format.info().mime),
        target_file_name,
    );
}

fn bytes_to_base64_uri(bytes: &[u8], mime: &str) -> String {
    let base64_string = base64::encode(bytes);
    let download_url = format!("data:{};base64,{}", mime, base64_string);
    download_url
}

fn download_file_with_uri(uri: String, filename: &str) {
    let window = web_sys::window().expect("Window should exist");
    let document = window.document().expect("Document should exist");

    let element = document
        .create_element("a")
        .expect("should be able to create element");

    let _ = element.set_attribute("href", uri.as_str());
    let _ = element.set_attribute("download", filename);

    let event = document
        .create_event("MouseEvents")
        .expect("should be able to call createEvent()");
    event.init_event_with_bubbles_and_cancelable("click", true, true);
    let _ = element.dispatch_event(&event);

    element.remove();
}
