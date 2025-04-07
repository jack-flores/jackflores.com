use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub reverse: bool,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let flex_class = if props.reverse {
        "lg:flex-row-reverse"
    } else {
        "lg:flex-row"
    };
    html! {
        <div class={format!("flex flex-col overflow-hidden rounded-md shadow-sm {}", flex_class)}>
            <div class="flex flex-col justify-center flex-1 p-6 dark:bg-gray-50">
                <span class="text-xs uppercase dark:text-gray-600">{ &props.subtitle}</span>
                <h3 class="text-3xl font-bold capitalize">{ &props.title }</h3>
                <p class="my-6 dark:text-gray-600">{ &props.description }</p>
            </div>
        </div>
    }
}
