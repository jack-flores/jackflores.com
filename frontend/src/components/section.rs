use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub title: String,
    pub subtitle: String,
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <div class="flex flex-col overflow-hidden rounded-md shadow-sm">
            <div class="flex flex-col justify-center flex-1 p-6 bg-gray-50">
                <span class="text-xs uppercase text-gray-600">{ &props.subtitle }</span>
                <h3 class="text-2xl font-bold capitalize">{ &props.title }</h3>
                <p class="my-6 text-gray-600">{ for props.children.iter() }</p>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionWrapperProps {
    #[prop_or_default]
    pub header: Option<String>,
    pub children: Children,
}

#[function_component(SectionsWrapper)]
pub fn sections_wrapper(props: &SectionWrapperProps) -> Html {
    let header = match &props.header {
        Some(text) => {
            html! { <h2 class="ml-10 mb-1 mt-3 text-gray-800 text-4xl font-bold capitalize underline decoration-double decoration-red-500">{ text }</h2> }
        }
        None => html! {},
    };
    html! {
        <>
            {header}
            <section class="p-4 bg-gray-100 text-gray-800">
                <div class="container mx-auto space-y-12">
                    { for props.children.iter() }
                </div>
            </section>
        </>
    }
}
