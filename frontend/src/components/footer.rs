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
            class="border-b-2 border-transparent hover:text-gray-100 hover:border-red-500 mx-1.5 sm:mx-6 hidden sm:flex"
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
        <div class="bg-gradient-to-r from-gray-800 via-red-950 to-gray-800 w-full mt-auto">
            <div class="container text-xs flex items-center justify-center p-5 mx-auto text-gray-400">
                <a href="https://github.com/jack-flores/jackflores.com" class="mr-auto font-bold hover:text-red-500 outline hover:outline-gray-100 text-gray-200 outline-red-500">{format!("© Copyright Jack Flores {}", chrono::Utc::now().year())}</a>
                <FooterItem href="https://github.com/jack-flores/jackflores.com"><Icon icon_id={IconId::BootstrapGithub}/></FooterItem>
                <FooterItem href="https://www.linkedin.com/in/jack-flores-51a875264/"><Icon icon_id={IconId::BootstrapLinkedin}/></FooterItem>
                <FooterItem href={email}><Icon icon_id={IconId::LucideMail}/></FooterItem>
                <a href="/contact" class="border-b-2 border-transparent hover:text-gray-100 hover:border-red-500 mx-1.5 sm:mx-6 sm:hidden xs:flex">{"Contact Me"}</a>
            </div>
        </div>
    }
}
