mod components;
mod pages;
mod router;

use components::{footer::*, images::*, links::*, nav::*, section::*};
use router::App;

use gloo_timers::callback::Interval;
use yew::prelude::*;

#[function_component(Home)]
fn home() -> Html {
    let attributes = [
        "Software Engineer",
        "Musician",
        "Sailor",
        "Leader",
        "Songwriter",
        "Rust Enthusiast",
        "Music Engineer",
        "Performer",
        "Fullstack Engineer",
        "Disc Golfer"
    ];

    let counter = use_state(|| 0);
    let counter_clone = counter.clone();

    use_effect(move || {
        let interval = Interval::new(3000, move || {
            if *counter_clone + 1 < attributes.len() {
                counter_clone.set(*counter_clone + 1);
            } else {
                counter_clone.set(0);
            }
        });

        move || drop(interval)
    });

    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar/>

            <ImageCloud>
                <Image src="/images/lark_portrait.jpeg" alt="College Sailing"></Image>
                <Image src="/images/media_day.jpg" alt="Tufts Media Day"></Image>
                <Image src="/images/jungle-2.jpeg" alt="Bad Neighbours at The Jungle"></Image>
            </ImageCloud>

            <div class="md:text-3xl sm:text-2xl text-lg flex justify-center">{"My name is"} {"\u{00a0}"} <p class="font-bold">{" Jack Flores "}</p> {", and I'm a\u{00a0}"} <span class="animate-pulse duration-100 font-bold underline decoration-double decoration-red-500">{ attributes[*counter] }</span>{"."}</div>

            <SectionsWrapper>
                <Section title="about me" subtitle="Engineer. Leader. Artist.">
                    {"I am a senior at Tufts University graduating this May with a BS in Computer Science and a minor in Engineering Management. After four years of balancing a rigorous course load, a varsity sport, leading a rock band, and working two incredible internships, my skillset and work ethic are exceptional. I have a demonstrated aptitude for leadership, having twice been voted co-captain by my teammates in varsity athletics."}<br/><br/>
                    {"I made this website to demonstrate my capabilities as a fullstack developer while providing an easily navigable portfolio. Everything you see was "} <b>{"built from scratch using Rust"}</b>{" -- my favorite programming language! Check out this project's "}
                    <PageBodyLink href="https://github.com/jack-flores/portfolio">{"GitHub repository linked here!"}</PageBodyLink>
                </Section>
                <Section title="skills" subtitle="Fullstack. Data. Teamwork.">
                    <b>{"Programming Languages:"}</b> {" Rust, Python, C, C++, Node.js, Next.js, React, Pandas"}<br/>
                    <b>{"Tools:"}</b> {" PostgreSQL, Git/GitHub/GitLab, Docker, AWS, Azure, MongoDB, Prisma, Linux, Arduino (hardware & programming)"}<br/>
                    <b>{"Methodologies:"}</b> {" CI/CD, Unit Testing, Integration Testing"}
                </Section>
                <Section title="projects" subtitle="Web. Embedded Systems. Graphics.">
                    <PageBodyLink href="/projects">{"Click here to visit my projects page!"}</PageBodyLink>
                </Section>
                <Section title="passions" subtitle="Songwriter. Athlete. Performer.">
                    {"I am a self-taught songwriter and music producer. I play every instrument you would see on a rock stage, and by myself, I have written, recorded, and produced dozens of songs. I also love performing music live and have played numerous shows with my band Bad Neighbours. I also love sailing, snowboarding, and playing disc golf"}
                </Section>
            </SectionsWrapper>
            <Footer/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
