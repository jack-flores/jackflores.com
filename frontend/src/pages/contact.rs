use crate::components;

use components::{footer::*, images::*, links::*, nav::*, section::*};

use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar/>

            <SectionsWrapper header="Contact Me">
                <Section title="Methods of Contact" subtitle="">
                        {"Email "} <b>{"(Preferred): "}</b><PageBodyLink href="mailto:jack.flores@tufts.edu">{"jack.flores@tufts.edu"}</PageBodyLink><br/>
                        {"Mobile Phone: "} <PageBodyLink href="tel:+19499331333">{"949-933-1333"}</PageBodyLink><br/>
                        {"Connect with me on "}<PageBodyLink href="https://www.linkedin.com/in/jack-flores-51a875264/">{"LinkedIn"}</PageBodyLink><br/>
                        <PageBodyLink href="https://github.com/jack-flores">{"GitHub"}</PageBodyLink>
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
