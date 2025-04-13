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
                    {"I used this project as an opportunity to show off my full-stack development skills while delivering an easily-navigable portfolio. Visit the "}<PageBodyLink href="https://github.com/jack-flores/jackflores.com">{"GitHub repository linked here!"}</PageBodyLink><br/><br/>
                    <b>{"Highlights:"}</b><br/>
                    <ul class="list-disc list-outside pl-4">
                        <li>{"Built Rust/Yew component modules for easy reproducibility and site-wide design consistency."}</li>
                        <li>{"Designed and constructed responsive and user-friendly web-pages using Yew components, HTML, and Tailwind CSS."}</li>
                        <li>{"Developed a REST API and an integration test suite with the Actix Web framework for Rust."} </li>
                        <li>{"Containerized the project with Docker and deployed with Fly.io."} </li>
                        <li>{"Implemented a CI/CD pipeline with GitHub actions to automatically deploy new changes."} </li>
                    </ul><br/>
                    <b>{"Tools Used: "}</b> {"Rust, Yew Framework (frontend), HTML, Tailwind CSS, Actix Web Framework (backend), GitHub, CI/CD -- GitHub Actions, Docker, Fly.io, Project Design, Project Management"}
                </Section>
                <Section title="DIY Gamma Ray Spectrometer" subtitle="September 2024 - May 2025">
                    {"For my Senior Capstone Project, I and a team of four other talented engineers developed a low-cost, easily reproducible alternative to traditional spectroscopic instruments."}<br/><br/>
                    <b>{"Highlights:"}</b><br/>
                    <ul class="list-disc list-outside pl-4">
                        <li>{"Created an embedded system that detects gamma ray events by utilizing analog signal processing circuitry."}</li>
                        <li>{"Designed, tested, and assembled a temperature-control system that uses a PID control-loop hosted on a "} <PageBodyLink href="https://www.pjrc.com/store/teensy41.html">{"Teensy 4.1 Microcontroller"}</PageBodyLink>{"."} </li>
                        <li>{"Simulated analog circuit design with LTspice."}</li>
                    </ul><br/>
                    <b>{"Tools Used: "}</b> {"C++, Arduino, Analog Circuitry, KiCAD, LTspice"}
                </Section>
                <Section title="Environment Mapping Simulation with Shaders" subtitle="November 2024">
                    {"Computer graphics project written in C++ using the OpenGL framework. Emphasized manual memory management, linear algebra and matrix transformation computations, and GPU interaction."}<br/><br/>
                    <b>{"Highlights:"}</b><br/>
                    <ul class="list-disc list-outside pl-4">
                        <li>{"Wrote a C++ program to render a spherical environment map in an OpenGL scene."}</li>
                        <li>{"Utilized shaders to coordinate data passing between the CPU and the GPU."} </li>
                        <li>{"Applied matrix transformations and linear algebra concepts to compute scene lighting and object positioning."}</li>
                    </ul><br/>
                    <b>{"Tools Used: "}</b> {"C++, OpenGL"}<br/><br/>
                    <b>{"Demo: "}</b><br/>
                    <video class="lg:w-[50%] sm:w-[70%] xs:w-[90%]" autoplay=true controls=true muted=true playsinline=true>
                        <source src="/images/ray-trace-demo.mp4" type="video/mp4"/>
                        {"Unable to show video -- your browser does not support the video tag."}
                    </video>

                </Section>
                <Section title="Homestart Fullstack Web Application" subtitle="September 2023 - April 2024">
                    {"I was a full-stack developer on a team for the "}<PageBodyLink href="https://jumbocode.org/">{"Tufts JumboCode Organization"}</PageBodyLink> {" supporting "} <PageBodyLink href="https://www.homestart.org/">{"Homestart"}</PageBodyLink> {", a Boston-based nonprofit. We built a web application to create surveys and collect and model data from survey results."}<br/><br/>
                    <b>{"Highlights:"}</b><br/>
                    <ul class="list-disc list-outside pl-4">
                        <li>{"Built a REST API with Next.js, Prisma, and MongoDB, emphasizing data format and consistency."} </li>
                        <li>{"Integrated React components with server-side logic in Next.js, using Prisma to query MongoDB and dynamically render data-driven web pages."}</li>
                        <li>{"Developed responsive and user-friendly web-pages adhering to design specifications using React, HTML, and Tailwind CSS."}</li>
                    </ul><br/>
                    <b>{"Tools Used: "}</b> {"Next.js, React, Prisma, MongoDB, GitHub, Tailwind CSS, HTML"}
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
