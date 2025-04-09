use crate::components;
use crate::pages;

use crate::Home;
use pages::contact::*;
use pages::experience::*;
use pages::projects::*;
use components::{footer::*, nav::*, section::*};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/experience")]
    Experience,
    #[at("/contact")]
    Contact,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Experience => html! { <Experience /> },
        Route::Contact => html! { <Contact /> },
        Route::Projects => html! { <Projects /> },
        Route::NotFound => html! { 
            <div class="flex flex-col min-h-screen">
                <NavBar/>
                <SectionsWrapper>
                    <Section title="404 -- Not Found" subtitle="">
                            {"This page does not exist!"}
                    </Section>
                </SectionsWrapper>

                <Footer/>
            </div>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}