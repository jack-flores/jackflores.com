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
            <div class="flex flex-col justify-center flex-1 p-6 bg-gradient-to-br from-red-100 via-gray-200 to-red-100">
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
            html! {
                <div class="bg-gradient-to-r from-red-100 via-gray-200 to-red-100"><h2 class="ml-10 mb-1 mt-3 text-gray-800 text-4xl font-bold capitalize underline decoration-double decoration-red-500">{ text }</h2></div>
            }
        }
        None => html! {},
    };
    html! {
        <>
            {header}
            <section class="p-4 text-gray-800 bg-gradient-to-br from-red-950 via-gray-800 to-red-950">
                <div class="container mx-auto space-y-8">
                    { for props.children.iter() }
                </div>
            </section>
        </>
    }
}
