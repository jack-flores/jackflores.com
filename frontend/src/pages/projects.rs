use crate::components;

use components::{footer::*, images::*, links::*, nav::*, section::*};

use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar/>

            <SectionsWrapper header="Projects">
                <Section title="Portfolio Website -- Where you are now!" subtitle="April 2025 - Present">
                    {"Visit the "}<PageBodyLink href="https://github.com/jack-flores/portfolio">{"GitHub repository linked here!"}</PageBodyLink><br/>
                    <b>{"Tools Used: "}</b> {"Rust, Yew Framework (frontend), Tailwind CSS, Actix Web Framework (backend), GitHub"}
                </Section>
                <Section title="DIY Gamma Ray Spectrometer" subtitle="September 2024 - May 2025">
                    <b>{"Tools Used: "}</b> {"C++, Arduino, Analog Circuitry, KiCAD"}
                </Section>
                <Section title="Ray Tracer" subtitle="November 2024">
                    <b>{"Tools Used: "}</b> {"C++, OpenGL"}
                </Section>
                <Section title="Homestart Fullstack Web Application" subtitle="September 2023 - April 2024">
                    <b>{"Tools Used: "}</b> {"Next.js, React, Prisma, MongoDB, GitHub, Tailwind CSS"}
                </Section>
            </SectionsWrapper>

            <ImageCloud>
                <Image src="/images/mitre_pres.jpg" alt="MITRE Presentation"></Image>
                <Image src="/images/lark-wing.jpeg" alt="Lark Sailing"></Image>
                <Image src="/images/homestart_presentation.jpg" alt="Jumbocode Homestart Team"></Image>
            </ImageCloud>

            <Footer/>
        </div>
    }
}
