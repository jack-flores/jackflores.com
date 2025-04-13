use crate::styles::*;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub title: String,
    pub subtitle: String,
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div class="flex flex-col overflow-hidden rounded-md shadow-sm">
            <div class={format!("flex flex-col justify-center flex-1 p-6 text-{prim} bg-gradient-to-br from-{bg_l_1} via-{bg_l_2} to-{bg_l_1}", bg_l_1 = COLOR_BG_LIGHT_1, bg_l_2 = COLOR_BG_LIGHT_2, prim = COLOR_TEXT_PRIMARY)}>
                <span class="text-xs uppercase">{ &props.subtitle }</span>
                <h3 class={format!("text-2xl font-bold capitalize text-{title}", title = COLOR_TEXT_TITLE)}>{ &props.title }</h3>
                <p class="my-6">{ for props.children.iter() }</p>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionWrapperProps {
    #[prop_or_default]
    pub header: Option<String>,
    pub children: Children,
}

#[function_component(SectionsWrapper)]
pub fn sections_wrapper(props: &SectionWrapperProps) -> Html {
    let header = match &props.header {
        Some(text) => {
            html! {
                <div class={format!("bg-gradient-to-r from-{bg_l_1} via-{bg_l_2} to-{bg_l_1}", bg_l_1 = COLOR_BG_LIGHT_1, bg_l_2 = COLOR_BG_LIGHT_2)}><h2 class={format!("ml-10 mb-1 mt-3 text-{title} text-4xl font-bold capitalize underline decoration-double decoration-{deco}", deco = COLOR_TEXT_DECORATION, title = COLOR_TEXT_TITLE)}>{ text }</h2></div>
            }
        }
        None => html! {},
    };
    html! {
        <>
            {header}
            <section class={format!("p-4 text-{prim} bg-gradient-to-br from-{bg_d_1} via-{bg_d_2} to-{bg_d_1}", bg_d_1 = COLOR_BG_DARK_1, bg_d_2 = COLOR_BG_DARK_2, prim = COLOR_TEXT_PRIMARY)}>
                <div class="container mx-auto space-y-8">
                    { for props.children.iter() }
                </div>
            </section>
        </>
    }
}
