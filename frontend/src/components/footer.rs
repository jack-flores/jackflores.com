use crate::constants::*;

use chrono::Datelike;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct FooterItemProps {
    pub href: String,
    pub children: Children,
}

#[function_component(FooterItem)]
pub fn footer_item(props: &FooterItemProps) -> Html {
    html! {
        <a
            href={props.href.clone()}
            class={format!("border-b-2 border-transparent hover:text-{nav_hov} hover:border-{deco} mx-1.5 sm:mx-6 hidden sm:flex", deco = COLOR_TEXT_DECORATION, nav_hov = COLOR_TEXT_NAV_HOVER)}
        >
            { for props.children.iter() }
        </a>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    let user = "jack.flores";
    let domain = "tufts.edu";
    let email = format!("mailto:{}@{}", user, domain);
    html! {
        <div class={format!("bg-gradient-to-r from-{bg_d_2} via-{bg_d_1} to-{bg_d_2} w-full mt-auto", bg_d_1 = COLOR_BG_DARK_1, bg_d_2 = COLOR_BG_DARK_2)}>
            <div class={format!("container text-xs flex items-center justify-center p-5 mx-auto text-{nav}", nav = COLOR_TEXT_NAV)}>
                <a href="https://github.com/jack-flores/jackflores.com" class={format!("mr-auto font-bold hover:text-{deco} outline hover:outline-{nav_hov} text-{logo} outline-{deco}", deco = COLOR_TEXT_DECORATION, logo = COLOR_TEXT_LOGO, nav_hov = COLOR_TEXT_NAV_HOVER)}>{format!("© Copyright Jack Flores {}", chrono::Utc::now().year())}</a>
                <FooterItem href="https://github.com/jack-flores/jackflores.com"><Icon icon_id={IconId::BootstrapGithub}/></FooterItem>
                <FooterItem href="https://www.linkedin.com/in/jack-flores-51a875264/"><Icon icon_id={IconId::BootstrapLinkedin}/></FooterItem>
                <FooterItem href={email}><Icon icon_id={IconId::LucideMail}/></FooterItem>
                <a href="/contact" class={format!("border-b-2 border-transparent hover:text-{nav_hov} hover:border-{deco} mx-1.5 sm:mx-6 sm:hidden xs:flex", deco = COLOR_TEXT_DECORATION, nav_hov = COLOR_TEXT_NAV_HOVER)}>{"Contact Me"}</a>
            </div>
        </div>
    }
}
