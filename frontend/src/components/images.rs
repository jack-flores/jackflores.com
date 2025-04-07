use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub src: String,
    pub alt: String,
}

#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    html! {
        <a href={props.src.clone()} class="group relative flex h-[25vh] xl:h-[40vh] [@media(max-height:640px)]:h-[50vh] items-end overflow-hidden rounded-lg bg-gray-100 shadow-lg md:col-span-2">
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
        <div class="columns-3 gap-4 space-y-4">
            { for props.children.iter() }
        </div>
    }
}
