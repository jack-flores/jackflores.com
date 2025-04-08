use crate::components;

use components::{footer::*, nav::*, section::*, images::*, links::*};

use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div>
            <NavBar>
                <NavLink href="/about">{"about me"}</NavLink>
                <NavLink href="/skills">{"skills"}</NavLink>
                <NavLink href="/projects">{"projects"}</NavLink>
                <NavLink href="/passions">{"passions"}</NavLink>
                <NavLink href="/contact">{"contact me"}</NavLink>
            </NavBar>

            <SectionsWrapper>
                <Section
                    id="about"
                    title="contact me"
                    subtitle=""
                    reverse={false}>
                        {"Email "} <b>{"(Preferred): "}</b><PageBodyLink href="mailto:jack.flores@tufts.edu">{"jack.flores@tufts.edu"}</PageBodyLink><br/>
                        {"Mobile Phone: "} <PageBodyLink href="tel:+19499331333">{"949-933-1333"}</PageBodyLink><br/>
                        <PageBodyLink href="https://www.linkedin.com/in/jack-flores-51a875264/">{"LinkedIn"}</PageBodyLink><br/>
                        <PageBodyLink href="https://github.com/jack-flores">{"GitHub"}</PageBodyLink>
                </Section>
            </SectionsWrapper>

            <ImageCloud>
                <Image src="/images/mitre_interns.jpeg" alt="MITRE Interns Summer 2024"></Image>
                <Image src="/images/lark_portrait.jpg" alt="College Sailing"></Image>
                <Image src="/images/as_interns.jpeg" alt="AS Software Interns Summer 2023"></Image>
            </ImageCloud>
            
            <Footer></Footer>
        </div>
    }
}