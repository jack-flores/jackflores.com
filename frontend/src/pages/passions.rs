use crate::components;

use components::{footer::*, images::*, links::*, nav::*, section::*};

use yew::prelude::*;

#[function_component(Passions)]
pub fn passions() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar/>

            <ImageCloud>
                <Image src="/images/lark_tack.jpg" alt="Lark Sailing"></Image>
                <Image src="/images/jungle-4.jpeg" alt="Bad Neighbours at The Jungle"></Image>
                <Image src="/images/sugarbush.jpeg" alt="Sugarbush with Friends"></Image>
            </ImageCloud>

            <SectionsWrapper header="Passions">
                <Section title="Music -- Recording, Production, Songwriting, and Performing" subtitle="2018 - present">
                    {"I am a self-taught musician, having mastered several instruments, recording and production techniques, and songwriting. While I am not considering a career in music, I am always developing new songs and exploring new techniques. I plan on publishing my some of my work to Spotify and other platforms by the end of 2025. "}
                    {"I am currently working on my debut album! "}<PageBodyLink href="https://drive.google.com/drive/folders/1jUtb_CqUrgH01aZ9syHS_tV8oysVUKHR?usp=drive_link">{"Click here to check out my album demos!"}</PageBodyLink><br/><br/>
                    {"In addition to my solo work, I am also the front-man of my band, "} <PageBodyLink href="https://www.instagram.com/badneighboursma">{"Bad Neighbours"}</PageBodyLink> {". We formed in 2022 at Tufts and have been playing shows in the Boston area since 2024. "}<PageBodyLink href="https://youtu.be/2n20fK8jSko?si=i-EGJC1l9pR7IiAz&t=1872">{"Click here to watch our 1/28/2025 live performance of an original song -- The Runaway!"}</PageBodyLink><br/><br/>
                    {"I am always looking to make new friends in the music community! If you would like to chat about music, "}<PageBodyLink href="/contact">{"contact me!"}</PageBodyLink><br/><br/>
                    <b>{"Instruments: "}</b> {"Vocals, acoustic guitar, electric guitar, drums, electric bass, keyboard, harmonica"}<br/>
                    <b>{"Skills: "}</b> {"Project management, Logic Pro, acoustic & electric instrument recording, vocal recording, live performance"}
                </Section>
                <Section title="Sailing" subtitle="2010 - present">
                    {"I have dedicated a significant amount of my life to competitive sailing, having competed in national and world championships as both a skipper and a crew. The past four years, I have been a dedicated member of the Tufts Varsity Sailing Team. For my senior year, I was elected as one of three Co-Captains of the Tufts team, a role that I have cherished and have been honored to serve."}<br/><br/>
                    <b>{"Disciplines: "}</b> {"Dinghy fleet racing, team racing, keelboat match racing, skiff sailing, team leadership"}
                </Section>
                <Section title="Being Outdoors" subtitle="2002 - present">
                    {"When I'm not coding, making music, or sailing, I love picking up new outdoor hobbies. Some notable examples include surfing, disc golfing, snowboarding, skateboarding, and hiking."}<br/>
                </Section>
            </SectionsWrapper>

            <Footer/>
        </div>
    }
}
