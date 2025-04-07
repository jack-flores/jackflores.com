use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub href: String,
    pub children: Children,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    html! {
        <a href={props.href.clone()} class="border-b-2 border-transparent hover:text-gray-100 hover:border-red-500 mx-1.5 sm:mx-6">
            { for props.children.iter() }
        </a>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    pub children: Children,
}

#[function_component(NavBar)]
pub fn nav_link(props: &NavBarProps) -> Html {
    html! {
        <nav class="bg-gray-800 shadow">
            <div class="container flex items-center justify-center p-5 mx-auto text-gray-400 capitalize">
                <a href="" class="mr-auto font-bold text-2xl hover:text-red-500 outline hover:outline-gray-100 text-gray-200 outline-red-500">{"jack flores"}</a>
                { for props.children.iter() }
            </div>
        </nav>
    }
}
