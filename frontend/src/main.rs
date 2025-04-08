mod components;
mod pages;
mod router;

use router::App;
use components::{footer::*, nav::*, section::*, images::*, links::*};

use yew::prelude::*;

#[function_component(Home)]
fn home() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar>
                <NavLink href="/about">{"about me"}</NavLink>
                <NavLink href="/experience">{"experience"}</NavLink>
                <NavLink href="/projects">{"projects"}</NavLink>
                <NavLink href="/passions">{"passions"}</NavLink>
                <NavLink href="/contact">{"contact me"}</NavLink>
            </NavBar>

            <ImageCloud>
                <Image src="/images/mitre_interns.jpeg" alt="MITRE Interns Summer 2024"></Image>
                <Image src="/images/lark_portrait.jpg" alt="College Sailing"></Image>
                <Image src="/images/as_interns.jpeg" alt="AS Software Interns Summer 2023"></Image>
            </ImageCloud>

            <div class="flex justify-center">{"My name is "} <b>{" Jack Flores "}</b> {", and I'm a "} <span class="animate-bounce font-bold">{" Software Engineer "}</span></div>

            <SectionsWrapper>
                <Section title="about me" subtitle="Engineer. Leader. Artist.">
                    {"After four years of balancing a computer science degree, a varsity sport, leading a rock band, and working two incredible internships, my skillset and work ethic are exceptional."}<br/><br/>
                    {"I made this website to demonstrate my capabilities as a fullstack developer while providing an easily navigable portfolio. Everything you see was "} <b>{"built from scratch using Rust"}</b>{" -- my favorite programming language! Check out this project's "}
                    <PageBodyLink href="https://github.com/jack-flores/portfolio">{"GitHub Repository linked here!"}</PageBodyLink>
                </Section>
                <Section title="skills" subtitle="Fullstack. Data. Teamwork.">
                    <b>{"Programming Languages:"}</b> {" Rust, Python, C, C++, Node.js, Next.js, React, Pandas"}<br/>
                    <b>{"Tools:"}</b> {" PostgreSQL, Git/GitHub/GitLab, Docker, AWS, Azure, MongoDB, Prisma, Linux, Arduino (hardware & programming)"}<br/>
                    <b>{"Methodologies:"}</b> {" CI/CD, Unit Testing, Integration Testing"}
                </Section>
                <Section title="projects" subtitle="Web. Embedded Systems. Graphics.">
                    {"DIY Gamma Ray Spectrometer, Rust Faxing Microservice, Homestart Web App, Ray Tracing Demo"}
                </Section>
                <Section title="passions" subtitle="Songwriter. Athlete. Performer.">
                    {"I am a self-taught songwriter and music producer. I play every instrument you would see on a rock stage, and by myself, I have written, recorded, and produced dozens of songs. I also love performing music live and have played numerous shows with my band Bad Neighbours. I also love sailing, snowboarding, and playing disc golf"}
                </Section>
            </SectionsWrapper>
            <Footer></Footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
