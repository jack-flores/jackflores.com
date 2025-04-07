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
            class="border-b-2 border-transparent hover:text-gray-100 hover:border-red-500 mx-1.5 sm:mx-6"
        >
            { for props.children.iter() }
        </a>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="bg-gray-800 shadow">
            <div class="container flex items-center justify-center p-5 mx-auto text-gray-400">
                <div class="mr-auto">{format!("© Copyright Jack Flores {}", chrono::Utc::now().year())}</div>
                <FooterItem href="https://www.linkedin.com/in/jack-flores-51a875264/">{"LinkedIn"}</FooterItem>
                <FooterItem href="mailto:jack.flores@tufts.edu">{"jack.flores@tufts.edu"}</FooterItem>
                <FooterItem href="tel:+19499331333">{"949-933-1333"}</FooterItem>
            </div>
        </div>
    }
}
