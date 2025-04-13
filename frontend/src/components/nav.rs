use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub href: String,
    pub children: Children,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    html! {
        <a href={props.href.clone()} class="border-b-2 border-transparent hover:text-gray-100 hover:border-red-500 mx-3">
            { for props.children.iter() }
        </a>
    }
}

#[function_component(NavBar)]
pub fn nav_link() -> Html {
    html! {
        <nav class="bg-gradient-to-r from-gray-800 via-red-950 to-gray-800">
            <div class="container flex items-center justify-between flex-wrap p-5 mx-auto text-gray-400 capitalize">
                <a href="/" class="xs:order-1 sm:order-none font-bold text-2xl hover:text-red-500 outline hover:outline-gray-100 text-gray-200 outline-red-500">{"jack flores"}</a>
                <div class="xs:order-2 sm:order-none flex flex-wrap justify-center sm:justify-end w-full sm:w-auto">
                    <NavLink href="/">{"about me"}</NavLink>
                    <NavLink href="/experience">{"experience"}</NavLink>
                    <NavLink href="/projects">{"projects"}</NavLink>
                    <NavLink href="/passions">{"passions"}</NavLink>
                    <NavLink href="/contact">{"contact"}</NavLink>
                </div>
            </div>

        </nav>
    }
}
