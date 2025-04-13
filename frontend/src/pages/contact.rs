use crate::components;

use components::{footer::*, images::*, links::*, nav::*, section::*};

use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Contact)]
pub fn contact() -> Html {
    let user = "jack.flores";
    let domain = "tufts.edu";
    let email = format!("mailto:{}@{}", user, domain);
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar/>

            <SectionsWrapper header="Let's Connect">
                <Section title="Methods of Contact" subtitle="">
                        <div class="flex items-center font-bold"><PageBodyLink href={email.clone()}><Icon icon_id={IconId::LucideMail}/></PageBodyLink><PageBodyLink href={email.clone()}>{"\u{00a0}Email (Preferred)"}</PageBodyLink></div><br/>
                        <div class="flex items-center font-bold"><PageBodyLink href="https://www.linkedin.com/in/jack-flores-51a875264/"><Icon icon_id={IconId::BootstrapLinkedin}/></PageBodyLink>{"\u{00a0}Connect with me on\u{00a0}"}<PageBodyLink href="https://www.linkedin.com/in/jack-flores-51a875264/">{"LinkedIn"}</PageBodyLink></div><br/>
                        <div class="flex items-center font-bold"><PageBodyLink href="https://github.com/jack-flores/"><Icon icon_id={IconId::BootstrapGithub}/></PageBodyLink>{"\u{00a0}Check out my\u{00a0}"}<PageBodyLink href="https://github.com/jack-flores/">{"GitHub"}</PageBodyLink></div><br/>
                </Section>
            </SectionsWrapper>

            <ImageCloud>
                <Image src="/images/mystic_fj_tack.jpeg" alt="FJ Sailing"></Image>
                <Image src="/images/jungle-1.jpg" alt="Live at The Jungle"></Image>
                <Image src="/images/harman_cup.jpg" alt="Harman Cup Winners"></Image>
            </ImageCloud>

            <Footer/>
        </div>
    }
}
