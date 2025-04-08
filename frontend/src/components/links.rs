use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageBodyLinkProps {
    pub href: String,
    pub children: Children,
}

#[function_component(PageBodyLink)]
pub fn page_body_link(props: &PageBodyLinkProps) -> Html {
    html! {
        <a href={props.href.clone()} class="border-2 border-transparent font-bold hover:text-red-500 hover:border-red-500">
        { for props.children.iter() }
        </a>
    }
}
