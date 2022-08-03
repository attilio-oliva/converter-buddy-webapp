use std::sync::Arc;
use stylist::css;

use crate::components::{DescriptionPopup, FormatFormComponent, InputFormComponent, ServiceProps};
use crate::services::conversions::ConversionService;
use crate::services::conversions::DynamicService;
use crate::theme::*;

use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use yew_feather::{github::Github, info::Info};

#[function_component(App)]
fn app(services: &ServiceProps) -> Html {
    let theme = use_context::<ThemeContext>().unwrap();

    let global_style = css!(
        r#"
        html {
                font-size: large;
                font-family:  "Helvetica", -apple-system, BlinkMacSystemFont, sans-serif;

                background-color: ${bg};
                color: ${font_color};
        }
        
        @supports (font-variation-settings: normal) {
          html {
            font-family: "Inter var", sans-serif;
          }
          h1, h2, h3, h4 {
            font-family: "Cantarell", sans-serif;
            font-weight: 800;
            font-variation-settings: "wght" 800;
          }

          h2 {
            font-size: 28px;
          }
        }

        body{
            padding: 0;
            margin: 0;
        }

        header {
            grid-template-columns: auto 1fr auto;
            display: grid;
            align-items: center;
            background-color: ${primary};
            color: ${font_color};
            padding: 16px;
            margin: 0;
        }

        a {
            text-decoration: none;
            color: ${font_color}
        }

        p a {
            text-decoration: underline;
            color: white;
        }

        .logo{
            height: 100%;
            width: 100%;
            object-fit: contain
        }
        
        .navbar-left {
            display: flex;
            grid-column: 1/2;

            height: 30px;
            padding-left: 16px
        }

        .navbar-right {
            display: flex;
            grid-column: 3/4;

            height: 30px;
            margin-left: auto;
            padding-left: 10px
        }

        .navbar-item {
            margin-left: 8px;
            margin-right: 8px;
            margin-top: 3px;
            margin-bottom: 3px;
        }

        .content-container {
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
            text-align: center;
        }

        .content {
            padding: calc(1% + 8px);
        }

        .content-header {
            margin-bottom: 0px;
        }

        .content-inner-box {
            padding: calc(1% + 5px);
        }

        ul{
            list-style-type: none;
        }

        /* custom buttons */   

        input {
            color: ${font_color};
            background-color: ${bg};
            border: 2px solid ${primary};
            border-radius: 16px;
            transition-duration: 0.4s;
            padding: 5px;
            padding-left: 20px;
            padding-right: 20px;
            font-size: 16px;
        }
          
        input:hover {
          background-color: ${primary};
          /*color: ${bg};*/
        }

        /* custom select */

        select {
            /* Reset */
            /*appearance: none;*/
            border: 0;
            outline: 0;
            font: inherit;
            /* Personalize */
            background-color: ${font_color};
            color: ${bg};
            border: 2px solid ${bg};
            border-radius: 3px;
            /*box-shadow: 0 0 1em 0 rgba(0, 0, 0, 0.2);*/
            cursor: pointer;
            margin: 16px;
        }
        /* <option> colors */
        option {
          color: ${bg};
          background-color: ${font_color};
        }
        /* Remove focus outline */
        select:focus {
          outline: none;
        }
        li {
            list-style: disc outside none;
            display: list-item;
        }

        /* Custom Modal
        * =============================== */
        .modal {
        opacity: 0;
        visibility: hidden;
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        text-align: left;
        background: rgba(0,0,0, .9);
        transition: opacity .25s ease;
        }

        .modal__bg {
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        cursor: pointer;
        }

        .modal-state {
        display: none;
        }

        .modal-state:checked + .modal {
        opacity: 1;
        visibility: visible;
        }

        .modal-state:checked + .modal .modal__inner {
        top: 0;
        }

        .modal__inner {
        transition: top .25s ease;
        position: absolute;
        top: -20%;
        right: 0;
        bottom: 0;
        left: 0;
        width: 40%;
        margin: auto;
        overflow: auto;
        background: ${primary};
        border-radius: 5px;
        padding: 1em 2em;
        height: 50%;
        }

        .modal__close {
        position: absolute;
        right: 1em;
        top: 1em;
        width: 1.1em;
        height: 1.1em;
        cursor: pointer;
        }

        .modal__close:after,
        .modal__close:before {
        content: '';
        position: absolute;
        width: 2px;
        height: 1.5em;
        background: #ccc;
        display: block;
        transform: rotate(45deg);
        left: 50%;
        margin: -3px 0 0 -1px;
        top: 0;
        }

        .modal__close:hover:after,
        .modal__close:hover:before {
        background: #aaa;
        }

        .modal__close:before {
        transform: rotate(-45deg);
        }

        .open-popup-btn {
            cursor:pointer;
        }

        @media screen and (max-width: 768px) {
            .modal__inner {
                width: 90%;
                height: 90%;
                box-sizing: border-box;
            }

            .navbar-left {
                padding-left: 6px;
            }
        }
        /* =============================== */

        "#,
        bg = theme.background_color.clone(),
        font_color = theme.font_color.clone(),
        primary = theme.primary_color.clone(),
    );

    html! {
        <>
        <Global css={global_style} />
        <header>
            <div class="navbar-left">
            <a href="/" class="logo">
                <img src="assets/images/logo.svg" alt="Converter Buddy" />
            </a>
            </div>
            <div class="navbar-right">
            <label for="compatibility-popup" class="navbar-item open-popup-btn" href="/">
                <Info/>
            </label>
            <a class="navbar-item" href="https://github.com/attilio-oliva/converter-buddy-webapp">
                <Github/>
            </a>
            </div>
        </header>
        <div class="content-container">
                <div class="content">
                    <InputFormComponent></InputFormComponent>
                </div>
                <div class="content">
                    <FormatFormComponent converter={services.converter.clone()}></FormatFormComponent>
                </div>
                //<PreviewComponent></PreviewComponent>
        </div>

        <DescriptionPopup></DescriptionPopup>
        </>
    }
}

#[styled_component(Root)]
pub fn root(services: &ServiceProps) -> Html {
    html! {
        <ThemeProvider>
            <App converter={services.converter.clone()}/>
        </ThemeProvider>
    }
}

pub fn serve(conversion_service: Arc<dyn ConversionService>) {
    let services = ServiceProps {
        converter: Some(DynamicService(conversion_service)),
    };
    yew::start_app_with_props::<Root>(services);
    //yew::Renderer::<Root>::new().render();
}
