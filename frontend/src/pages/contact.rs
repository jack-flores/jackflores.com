use crate::components;

use components::{footer::*, nav::*, section::*, images::*, links::*};

use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar>
                <NavLink href="/about">{"about me"}</NavLink>
                <NavLink href="/skills">{"skills"}</NavLink>
                <NavLink href="/projects">{"projects"}</NavLink>
                <NavLink href="/passions">{"passions"}</NavLink>
                <NavLink href="/contact">{"contact me"}</NavLink>
            </NavBar>

            <SectionsWrapper>
                <Section title="contact me" subtitle="">
                        {"Email "} <b>{"(Preferred): "}</b><PageBodyLink href="mailto:jack.flores@tufts.edu">{"jack.flores@tufts.edu"}</PageBodyLink><br/>
                        {"Mobile Phone: "} <PageBodyLink href="tel:+19499331333">{"949-933-1333"}</PageBodyLink><br/>
                        <PageBodyLink href="https://www.linkedin.com/in/jack-flores-51a875264/">{"LinkedIn"}</PageBodyLink><br/>
                        <PageBodyLink href="https://github.com/jack-flores">{"GitHub"}</PageBodyLink>
                </Section>
            </SectionsWrapper>

            <ImageCloud>
                <Image src="/images/mystic_fj_tack.jpg" alt="FJ Sailing"></Image>
                <Image src="/images/jungle-1.jpg" alt="Live at The Jungle"></Image>
                <Image src="/images/harman_cup.jpg" alt="Harman Cup Winners"></Image>
            </ImageCloud>
            
            <Footer></Footer>
        </div>
    }
}