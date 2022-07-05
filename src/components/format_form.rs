use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use converter_buddy::converter_info;
use converter_buddy::format::{self, Format};
use gloo_console::log;
use gloo_file::callbacks::FileReader;
use itertools::Itertools;
use web_sys::{Event, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Component, Context, Html, Properties};
use yewdux::prelude::*;

use super::FileList;
use crate::services::conversions::{injector, DynamicService};

pub enum Msg {
    TargetFormat(Option<Format>),
    UpdateFileList(Rc<FileList>),
    SubmitConversion,
    LoadFile(String, Vec<u8>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct ServiceProps {
    pub converter: Option<DynamicService>,
}

pub struct FormatFormComponent {
    source_format: Option<Format>,
    target_format: Option<Format>,
    readers: HashMap<String, FileReader>,
    files: Rc<FileList>,
    #[allow(dead_code)]
    dispatch: Dispatch<FileList>,
}

impl FormatFormComponent {
    pub fn guess_source_format(file_name: String) -> Option<Format> {
        let ext = file_name.split('.').last();
        ext.and_then(format::from_extension)
    }
    pub fn supported_formats(&self) -> Vec<Format> {
        match self.source_format {
            Some(source_format) => converter_info::from_format(source_format).supported_formats(),
            None => vec![],
        }
    }
}

impl<'a> Component for FormatFormComponent {
    type Message = Msg;
    type Properties = ServiceProps;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::UpdateFileList);
        let dispatch = Dispatch::<FileList>::subscribe(move |val| callback.emit(val));

        Self {
            source_format: None,
            target_format: None,
            readers: HashMap::default(),
            files: dispatch.get(),
            dispatch,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateFileList(files) => {
                self.files = files;
                if self.files.list.is_empty() {
                    self.source_format = None;
                } else {
                    let file_name = self.files.list.first().unwrap().name();
                    self.source_format = FormatFormComponent::guess_source_format(file_name);
                }
                true
            }
            Msg::TargetFormat(format) => {
                //log!(format!("{:?}",format));
                self.target_format = format;
                true
            }
            Msg::SubmitConversion => {
                let target_format = self.target_format.expect("Target format is not selected");
                if !self.supported_formats().contains(&target_format) {
                    log!("Unsupported conversion");
                    return false;
                }
                for file in self.files.list.iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        gloo_file::callbacks::read_as_bytes(file, move |res| {
                            let data = res.expect("failed to read file");
                            link.send_message(Msg::LoadFile(file_name, data))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::LoadFile(file_name, data) => {
                let target_format = self.target_format.expect("Target format is not found");

                convert_and_download(
                    ctx.props()
                        .converter
                        .as_ref()
                        .unwrap_or(&DynamicService(injector::get_dummy_service())),
                    &file_name,
                    &data,
                    &target_format,
                );
                self.readers.remove(&file_name);
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
                <div>
                <h2 class="box-header"> { "Desired format" } </h2>
                <div class={"content-inner-box"}>
                        //<label>{"Format"}</label>
                        <select onchange={ctx.link().callback(|e: Event| {
                            let target: HtmlInputElement = e.target_unchecked_into();
                            let format = target.value();
                            let format = match format.as_str() {
                                "" => None,
                                _ => Some(Format::from_str(&format).unwrap()),
                            };
                            Msg::TargetFormat(format)
                        })}>
                            <option value="">{"Select a format"}</option>
                            {
                                FormatFormComponent::view_list(self.source_format)
                            }
                        </select>
                </div>
                <div class={"content-inner-box"}>
                    <input type="button" onclick={ctx.link().callback(|_| Msg::SubmitConversion)} value="Convert"/>
                </div>

            </div>
        }
    }
}

impl FormatFormComponent {
    pub fn view_list(source_format: Option<Format>) -> Html {
        match source_format {
            Some(format) => {
                let converter = converter_info::from_format(format);
                let formats = converter.supported_formats();
                let categorized_formats = FormatFormComponent::format_map(&formats);
                // This sort will cost memory and time, a code rework is needed to avoid that, if possible
                // It is used because HashMap does not garantee any order on iteration, causing the list to randomly change items' order
                categorized_formats.keys().sorted().map(|category| {
                    html! {
                        <optgroup label={category.to_string()}>
                        {
                            categorized_formats[category].iter().map(|format| {
                                html! {
                                    <option value={format.to_string()}>{format.to_string()}</option>
                                }
                            }).collect::<Html>()
                        }
                        </optgroup>
                    }
                }).collect::<Html>()
            }
            None => html! {<></>},
        }
    }

    fn format_map(formats: &[Format]) -> HashMap<String, Vec<&Format>> {
        let mut map = HashMap::new();
        for format in formats {
            let category = FormatFormComponent::mime_to_category(format.info().mime);
            map.entry(category).or_insert(vec![]).push(format);
        }
        map
    }

    fn mime_to_category(mime: &str) -> String {
        if mime.contains("text") {
            String::from("Text")
        } else if mime.contains("image") {
            String::from("Image")
        } else if mime.contains("audio") {
            String::from("Audio")
        } else if mime.contains("video") {
            String::from("Video")
        } else if mime.contains("application") {
            String::from("Document")
        } else {
            String::from("Other")
        }
    }
}

fn convert_and_download(
    service: &DynamicService,
    file_name: &str,
    data: &Vec<u8>,
    target_format: &Format,
) {
    let mut output = Vec::<u8>::new();
    let source_path = PathBuf::from(file_name.to_string());
    let source_ext = source_path.extension().unwrap().to_str().unwrap();

    let target_format_ext = target_format.info().preferred_extension;
    let target_path = PathBuf::from(file_name.to_string()).with_extension(target_format_ext);
    let target_file_name = target_path.as_os_str().to_str().unwrap();

    format::from_extension(source_ext)
        .map(|source_format| {
            //let converter = converter_info::from_format(format);
            //converter.process(data, &mut output, *target_format)
            service
                .0
                .process(data, &mut output, source_format, *target_format)
        })
        .unwrap()
        .expect("Conversion failed");
    download_file_with_uri(
        bytes_to_uri_b64(&output, target_format.info().mime),
        target_file_name,
    );
}

fn bytes_to_uri_b64(bytes: &[u8], mime: &str) -> String {
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
