use crate::components;

use components::{footer::*, images::*, links::*, nav::*, section::*};

use yew::prelude::*;

#[function_component(Passions)]
pub fn passions() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar/>

            <SectionsWrapper header="Passions">
                <Section title="Music -- Production, Recording, Songwriting, and Performing" subtitle="2018 - present">
                    {"I am a self-taught musician, having mastered several instruments, recording and production techniques, and songwriting. I love performing with my band, Bad Neighbours in the Boston area. "}
                    {"I am currently working on my debut album! "}<PageBodyLink href="https://drive.google.com/drive/folders/1jUtb_CqUrgH01aZ9syHS_tV8oysVUKHR?usp=drive_link">{"Click here to check out my album demos!"}</PageBodyLink><br/>
                    <b>{"Instruments: "}</b> {"Vocals, acoustic guitar, electric guitar, drums, electric bass, keyboard, harmonica"}<br/>
                    <b>{"Skills: "}</b> {"Project Management, Logic Pro, Acoustic & Electric Instrument Recording, Vocal Recording"}
                </Section>
                <Section title="Sailing" subtitle="2010 - present">
                    {"I have dedicated a significant amount of my life to competitive sailing, having competed in national and world championships as both a skipper and a crew. The past four years, I have been a dedicated member of the Tufts Varsity Sailing Team. I was elected as a Co-Captain of the Tufts team for my senior year."}<br/>
                    <b>{"Disciplines: "}</b> {"Dinghy fleet racing, team racing, keelboat match racing, skiff sailing"}
                </Section>
                <Section title="Being Outdoors" subtitle="2002 - present">
                    {"When I'm not coding, making music, or sailing, I love picking up new outdoor hobbies. Some notable examples include surfing, disc golfing, snowboarding, skateboarding, and hiking."}<br/>
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
