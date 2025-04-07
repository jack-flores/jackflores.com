mod components;

use components::{footer::*, nav::*, section::*};

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <nav class="bg-gray-800 shadow">
                <div class="container flex items-center justify-center p-5 mx-auto text-gray-400 capitalize">
                    <a href="" class="mr-auto font-bold text-2xl hover:text-red-500 outline hover:outline-gray-100 text-gray-200 outline-red-500">{"jack flores"}</a>
                    <NavLink href="#about">{"about me"}</NavLink>
                    <NavLink href="#skills">{"skills"}</NavLink>
                    <NavLink href="#projects">{"projects"}</NavLink>
                    <NavLink href="#passions">{"passions"}</NavLink>
                    <NavLink href="#contact">{"contact me"}</NavLink>
                </div>
            </nav>

            <section class="p-4 lg:p-8 dark:bg-gray-100 dark:text-gray-800">
                <div class="container mx-auto space-y-12">
                    <Section
                        id="about"
                        title="about me"
                        subtitle="Engineer. Leader. Artist."
                        description="After four years of balancing a computer science degree, a varsity sport, leading a rock band, and working two incredible internships, I couldn't be more grateful to have had such an incredible experience at Tufts."
                        reverse={false}>
                    </Section>
                    <Section
                        id="skills"
                        title="skills"
                        subtitle="Fullstack. Data. Teamwork."
                        description="Programming Languages: Rust, Python, C, C++, Node.js, Next.js, React, Pandas. Tools: PostgreSQL, Git/GitHub/GitLab, Docker, AWS, Azure, MongoDB, Prisma, Linux, Arduino (hardware & programming). Methodologies: CI/CD, Unit Testing, Integration Testing"
                        reverse={true}>
                    </Section>
                    <Section
                        id="projects"
                        title="projects"
                        subtitle="Web. Embedded Systems. Graphics."
                        description="DIY Gamma Ray Spectrometer, Rust Faxing Microservice, Homestart Web App, Ray Tracing Demo"
                        reverse={false}>
                    </Section>
                    <Section
                        id="passions"
                        title="passions"
                        subtitle="Songwriter. Athlete. Performer."
                        description="I am a self-taught songwriter and music producer. I play every instrument you would see on a rock stage, and by myself, I have written, recorded, and produced dozens of songs. I also love performing music live and have played numerous shows with my band Bad Neighbours. I also love sailing, snowboarding, and playing disc golf"
                        reverse={true}>
                    </Section>

                </div>
            </section>
            <Footer></Footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
