use converter_buddy::{converter::Converter, format::Format};
use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::components::format_form::conv_supported_formats;

pub struct DescriptionPopup;

impl Component for DescriptionPopup {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <input class="modal-state" id="compatibility-popup" type="checkbox" />
                <div class="modal">
                <label class="modal__bg" for="compatibility-popup"></label>
                <div class="modal__inner">
                    <label class="modal__close" for="compatibility-popup"></label>
                    <h2>{"What is this website?"}</h2>
                    {Self::description()}
                    <h3>{"Currently supported formats"}</h3>
                    {Self::compatibility_list()}
                </div>
                </div>
            </>
        }
    }
}

impl DescriptionPopup {
    pub fn description() -> Html {
        html! {
            <>
            <p>
                <b>{"Converter Buddy"}</b>
                {" is a file conversion utility. This website uses Converter Buddy in your "}
                <b>{"local browser"}</b>
                {" rather than a server, so that your "}
                <b>{"data is not shared online"}</b>
                {". The whole project is "}
                <b>{"open-source"}</b>
                {" with a MIT license and can be found on "}
                <a href="https://github.com/attilio-oliva/converter-buddy-webapp">{"GitHub"}</a>
            </p>
            </>
        }
    }

    pub fn compatibility_list() -> Html {
        html! {
            <>
            <ul>
                {
                    Format::iter().map(|supported_format| {
                        let converter = Converter::try_from(supported_format);
                        match converter {
                            Ok(c) => html! {
                            <li>
                            {
                                format!("{} → {:?}", supported_format, conv_supported_formats(c))
                            }
                            </li>
                        },
                            Err(_) => html!{<></>},
                        }

                    }).into_iter().collect::<Html>()
                }
            </ul>
            </>
        }
    }
}
