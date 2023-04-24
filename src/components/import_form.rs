use dioxus::prelude::*;

pub fn ImportForm(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "content" }
    })
}
/*
pub enum Msg {
    InputFiles(Vec<File>),
    DragOver(DragEvent),
}

pub struct InputFormComponent {
    files: Vec<File>,
    dispatch: Dispatch<FileList>,
}

impl Component for InputFormComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<FileList>::new();
        Self {
            files: vec![],
            dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputFiles(files) => {
                self.files = files.clone();
                self.dispatch.set(FileList { list: files });
                true
            }
            Msg::DragOver(event) => {
                event.prevent_default();
                event.stop_propagation();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div ondrop={ctx.link().callback(move |event: DragEvent| {
                event.prevent_default();
                event.stop_propagation();

                let files = Self::files_from_drag(event);

                Msg::InputFiles(files)
            })}
            ondragover={ctx.link().callback(move |event: DragEvent| {
                Msg::DragOver(event)
            })}>
                <h2 class="content-header">{ "Select or drop files to convert" }</h2>
                <div class="content-inner-box">
                if self.files.is_empty() {
                    <p>{ "No file selected" }</p>
                } else {
                    <p>{ "Selected files:" }</p>
                    { for self.files.iter().map(|f| Self::view_file(&f.name())) }
                }
                </div>
                <div class="content-inner-box">
                    <input id="file-button" hidden=true type="file" multiple=true onchange={ctx.link().callback(move |e: Event| {
                            let files = Self::files_from_button(e);
                            Msg::InputFiles(files)
                        })}
                    />
                    <input type="button" onclick={ctx.link().batch_callback(move |_| {
                        Self::click_file_button();
                        None
                    })} value="Select files" />
                </div>
            </div>
        }
    }
}

impl InputFormComponent {
    fn view_file(data: &str) -> Html {
        html! {
            <li>{ data }</li>
        }
    }

    fn files_from_drag(event: DragEvent) -> Vec<File> {
        let dt = event.data_transfer().unwrap();
        let js_file_list = dt.files();

        match js_file_list.as_ref() {
            Some(js_file_list) => {
                let mut files = vec![];
                for i in 0..js_file_list.length() {
                    let file: File = js_file_list.item(i).unwrap().into();
                    files.push(file);
                }
                files
            }
            None => {
                vec![]
            }
        }
    }

    fn files_from_button(event: Event) -> Vec<File> {
        let mut result = Vec::new();
        let input: HtmlInputElement = event.target_unchecked_into();
        if let Some(files) = input.files() {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        result
    }

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
}
*/
