use chrono::Datelike;
use yew::prelude::*;

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
    html! {
        <div class="bg-gray-800 shadow">
            <div class="container text-xs flex items-center justify-center p-5 mx-auto text-gray-400">
                <a href="https://github.com/jack-flores/portfolio" class="mr-auto font-bold hover:text-red-500 outline hover:outline-gray-100 text-gray-200 outline-red-500">{format!("© Copyright Jack Flores {}", chrono::Utc::now().year())}</a>
                <a href="https://github.com/jack-flores/portfolio" class="hidden md:flex mr-auto border-b-2 border-transparent hover:text-gray-100 hover:border-red-500">{"GitHub Repository"}</a>
                <FooterItem href="https://www.linkedin.com/in/jack-flores-51a875264/">{"LinkedIn"}</FooterItem>
                <FooterItem href="mailto:jack.flores@tufts.edu">{"jack.flores@tufts.edu"}</FooterItem>
                <FooterItem href="tel:+19499331333">{"949-933-1333"}</FooterItem>
                <a href="/contact" class="sm:hidden xs:flex ml-auto">{"Contact Me"}</a>
            </div>
        </div>
    }
}
