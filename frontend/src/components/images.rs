use crate::styles::{COLOR_BG_DARK_1, COLOR_BG_DARK_2};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub src: String,
    pub alt: String,
}

#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    html! {
        <a href={props.src.clone()} class="group relative flex h-[25vh] xl:h-[40vh] [@media(max-height:640px)]:h-[50vh] items-end overflow-hidden rounded-lg shadow-lg md:col-span-2">
            <img
                class="absolute inset-0 h-full w-full object-cover object-center transition duration-200 group-hover:scale-110"
                src={props.src.clone()}
                alt={props.alt.clone()}/>
        </a>
    }
}

#[derive(Properties, PartialEq)]
pub struct ImageCloudProps {
    pub children: Children,
}

#[function_component(ImageCloud)]
pub fn image_cloud(props: &ImageCloudProps) -> Html {
    html! {
        <div class={format!("sm:columns-3 xs:columns-1 gap-4 space-y-4 bg-gradient-to-br from-{bg_d_1} via-{bg_d_2} to-{bg_d_1}", bg_d_1 = COLOR_BG_DARK_1, bg_d_2 = COLOR_BG_DARK_2)}>
            { for props.children.iter() }
        </div>
    }
}
