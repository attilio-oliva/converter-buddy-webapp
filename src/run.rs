use std::sync::Arc;
use stylist::css;

use crate::components::InputFormComponent;
use crate::components::{FormatFormComponent, ServiceProps};
use crate::services::conversions::ConversionService;
use crate::services::conversions::DynamicService;
use crate::theme::*;

use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use yew_feather::github::Github;

#[function_component(App)]
fn app(services: &ServiceProps) -> Html {
    let theme = use_context::<ThemeContext>().unwrap();

    let global_style = css!(
        r#"
        html, body {
                font-size: large;
                font-family: 'myFont', sans-serif;
                padding: 0;
                margin: 0;

                background-color: ${bg};
                color: ${font_color};
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
        
        .navbar-left {
            display: flex;
            grid-column: 1/2;
            padding-left: 10px
        }

        .navbar-right {
            display: flex;
            grid-column: 3/4;

            margin-right: 10px;
            margin-left: auto;
            padding-left: 10px
        }

        .navbar-item {
            margin-left: 16px;
            margin-right: 16px;
        }

        .content-container {
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
            padding: calc(2%, 24px);
            text-align: center;
        }

        .content-box {
            padding: calc(1% + 16px);
        }

        .box-header {
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
            border: 1px solid ${primary};
            border-radius: 16px;
            transition-duration: 0.4s;
            padding: 5px;
            padding-left: 20px;
            padding-right: 20px;
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
            <a href="/">
                <img src="assets/images/logo.svg" alt="Converter Buddy" />
            </a>
            </div>
            <div class="navbar-right">
            //<a class="navbar-item" href="/">
            //    <Info/>
            //</a>
            <a class="navbar-item" href="https://github.com/attilio-oliva/converter-buddy-webapp">
                <Github/>
            </a>
            </div>
        </header>
        <div class="content-container">
                <div class="content-box">
                    <InputFormComponent></InputFormComponent>
                </div>
                <div class="content-box">
                    <FormatFormComponent converter={services.converter.clone()}></FormatFormComponent>
                </div>
                //<PreviewComponent></PreviewComponent>
        </div>
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
