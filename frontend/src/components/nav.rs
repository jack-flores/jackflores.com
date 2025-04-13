use crate::constants::*;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub href: String,
    pub children: Children,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    html! {
        <a href={props.href.clone()} class={format!("border-b-2 border-transparent hover:text-{nav_hov} hover:border-{deco} mx-3", deco = COLOR_TEXT_DECORATION, nav_hov = COLOR_TEXT_NAV_HOVER)}>
            { for props.children.iter() }
        </a>
    }
}

#[function_component(NavBar)]
pub fn nav_link() -> Html {
    html! {
        <nav class={format!("bg-gradient-to-r from-{bg_d_2} via-{bg_d_1} to-{bg_d_2}", bg_d_1 = COLOR_BG_DARK_1, bg_d_2 = COLOR_BG_DARK_2)}>
            <div class={format!("container flex items-center justify-between flex-wrap p-5 mx-auto text-{nav} capitalize", nav = COLOR_TEXT_NAV)}>
                <a href="/" class={format!("xs:order-1 sm:order-none font-bold text-2xl hover:text-{deco} outline hover:outline-{nav_hov} text-{logo} outline-{deco}", deco = COLOR_TEXT_DECORATION, logo = COLOR_TEXT_LOGO, nav_hov = COLOR_TEXT_NAV_HOVER)}>{"jack flores"}</a>
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
