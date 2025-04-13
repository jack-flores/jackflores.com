use crate::COLOR_TEXT_DECORATION;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageBodyLinkProps {
    pub href: String,
    pub children: Children,
}

#[function_component(PageBodyLink)]
pub fn page_body_link(props: &PageBodyLinkProps) -> Html {
    html! {
        <a href={props.href.clone()} class={format!("border-2 border-transparent font-bold hover:text-{deco} hover:border-{deco}", deco = COLOR_TEXT_DECORATION)}>
        { for props.children.iter() }
        </a>
    }
}
