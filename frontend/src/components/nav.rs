use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub href: String,
    pub children: Children,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    html! {
        <a
            href={props.href.clone()}
            class="border-b-2 border-transparent hover:text-gray-100 hover:border-red-500 mx-1.5 sm:mx-6"
        >
            { for props.children.iter() }
        </a>
    }
}
