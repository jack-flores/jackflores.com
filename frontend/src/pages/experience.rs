use crate::components;

use components::{footer::*, nav::*, section::*, images::*, links::*};

use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar/>

            <SectionsWrapper>
                <Section title="The MITRE Corporation -- Computer Science Intern" subtitle="June - August 2024 | Overland Park, KS">
                    <PageBodyLink href="https://www.as-software.com/">{"AS Software"}</PageBodyLink>
                    <b>{"Tools Used:"}</b> {"Python, Node.js, Git/GitHub/GitLab, AWS, CI/CD -- GitHub Actions, Unit Testing, Integration Testing"}
                </Section>
                <Section title="AS Software -- Software Engineer Intern" subtitle="May - August 2023 | Bedford, MA">
                    <b>{"Tools Used:"}</b> {"Rust, PostgreSQL, Docker, Git/GitHub, Azure"}
                </Section>
            </SectionsWrapper>

            <ImageCloud>
                <Image src="/images/mitre_interns.jpeg" alt="MITRE Interns Summer 2024"></Image>
                <Image src="/images/lark_portrait.jpg" alt="College Sailing"></Image>
                <Image src="/images/as_interns.jpeg" alt="AS Software Interns Summer 2023"></Image>
            </ImageCloud>
            
            <Footer/>
        </div>
    }
}
