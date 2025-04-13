mod components;
mod pages;
mod router;
mod styles;

use components::{footer::*, images::*, links::*, nav::*, section::*};
use router::App;
use styles::{COLOR_BG_LIGHT_1, COLOR_TEXT_DECORATION, COLOR_TEXT_TITLE};

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
        "Full-Stack Developer",
        "Disc Golfer",
    ];

    let counter = use_state(|| 0);
    let counter_clone = counter.clone();

    use_effect(move || {
        let interval = Interval::new(4000, move || {
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

            <div class={format!("md:text-3xl sm:text-2xl text-lg text{title} font-bold flex flex-wrap justify-center bg-gradient-to-r from-{bg_l_1} via-gray-200 to-{bg_l_1} p-1", bg_l_1 = COLOR_BG_LIGHT_1, title = COLOR_TEXT_TITLE)}>
                <h2>{"My name is Jack Flores, and I'm a\u{00a0}"}</h2>
                <h2 class={format!("inline-block animate-typing overflow-hidden whitespace-nowrap border-r-4 border-r-{title} underline decoration-double decoration-{deco}", deco = COLOR_TEXT_DECORATION, title = COLOR_TEXT_TITLE)}>
                    {{ attributes[*counter] }}{"."}
                </h2>
            </div>


            <SectionsWrapper>
                <Section title="about me" subtitle="Engineer. Leader. Artist.">
                    {"I am a senior at Tufts University graduating this May with a Bachelor of Science in Computer Science and a minor in Engineering Management. After four years of balancing a rigorous course load, a varsity sport, leading a rock band, and working two incredible internships, my skillset and work ethic are exceptional. I have a demonstrated aptitude for leadership, having twice been voted co-captain by my teammates in varsity athletics."}<br/><br/>
                    {"I built this website to demonstrate my capabilities as a fullstack developer while providing an easily navigable portfolio. Everything you see was "} <b>{"built from scratch using Rust"}</b>{" -- my favorite programming language! Check out this project's "}
                    <PageBodyLink href="https://github.com/jack-flores/jackflores.com">{"GitHub repository linked here!"}</PageBodyLink>
                </Section>
                <Section title="skills" subtitle="Fullstack. Data. Teamwork.">
                    <div class="text-lg"><PageBodyLink href="/experience">{"Click here to read more about my work experience."}</PageBodyLink></div><br/>
                    <b>{"Languages:"}</b> {" Rust, C, C++, Python, Node.js, Next.js, React, HTML, Tailwind CSS, SQL"}<br/>
                    <b>{"Tools:"}</b> {" PostgreSQL, Git/GitHub/GitLab, Docker, AWS, Azure, MongoDB, Prisma, Linux, Arduino, Pandas"}<br/>
                    <b>{"Methodologies:"}</b> {" CI/CD, Unit Testing, Integration Testing, Agile"}
                </Section>
                <Section title="projects" subtitle="Web. Embedded Systems. Graphics.">
                    <PageBodyLink href="/projects">{"Click here to visit my projects page!"}</PageBodyLink>
                </Section>
            </SectionsWrapper>

            <Footer/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
