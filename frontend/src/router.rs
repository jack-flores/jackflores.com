use crate::components;
use crate::pages;

use crate::Home;
use pages::contact::Contact;
use components::{footer::*, nav::*, section::*};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { 
            <div class="flex flex-col min-h-screen">
                <NavBar>
                    <NavLink href="/about">{"about me"}</NavLink>
                    <NavLink href="/experience">{"experience"}</NavLink>
                    <NavLink href="/projects">{"projects"}</NavLink>
                    <NavLink href="/passions">{"passions"}</NavLink>
                    <NavLink href="/contact">{"contact me"}</NavLink>
                </NavBar>
                <SectionsWrapper>
                    <Section title="404 -- Not Found" subtitle="">
                            {"This page does not exist!"}
                    </Section>
                </SectionsWrapper>

                <Footer></Footer>
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