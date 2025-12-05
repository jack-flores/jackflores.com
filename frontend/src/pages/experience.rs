use crate::components;

use components::{footer::*, images::*, links::*, nav::*, section::*};

use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <NavBar/>

            <SectionsWrapper header="Work Experience">
                <Section title="Onwards HR -- Associate Software Engineer" subtitle="Decemebr 2025 - Present | Remote">
                    {"My current role! I have been working for "}<PageBodyLink href="https://www.onwardshr.com/">{"Onwards HR"}</PageBodyLink>{" since December 2025!"}<br/><br/>

                    <b>{"Highlights from the work I've done at Onwards:"}</b><br/>
                    <ul class="list-disc list-outside pl-4">
                        <li>{"Wrote software in Typescript."}</li>
                    </ul><br/>
                    <b>{"Tools Used: "}</b> {"Typescript, Node.js, PostgreSQL, Git/GitHub, AWS, CI/CD"}
                </Section>
                <Section title="The MITRE Corporation -- Computer Science Intern" subtitle="June - August 2024 | Bedford, MA">
                    {"At "} <PageBodyLink href="https://www.mitre.org/">{"The MITRE Corporation"}</PageBodyLink> {", I interned within the Center For Securing the Homeland and worked on projects supporting the "} <PageBodyLink href="https://www.dhs.gov/science-and-technology/hssedi">{"Homeland Security Systems Engineering and Development Institute FFRDC"} </PageBodyLink> {". I worked on two renowned projects -- "} <PageBodyLink href="https://www.cve.org/">{"Common Vulnerabilities and Exposures (CVE)"}</PageBodyLink> {", and "} <PageBodyLink href="https://cwe.mitre.org/">{"Common Weakness Enumeration (CWE)"}</PageBodyLink> {". I was honored to contribute to two crucial projects that are having a positive impact in the world."}<br/><br/>

                    <b>{"Highlights from the work I did at MITRE:"}</b><br/>
                    <ul class="list-disc list-outside pl-4">
                        <li>{"Implemented, tested, and delivered a new feature for the "} <PageBodyLink href="https://github.com/CVEProject/cve-services/">{"CVE Services API"}</PageBodyLink> {" to track API usage. "} <PageBodyLink href="https://github.com/CVEProject/cve-services/pull/1262">{"View my merged pull request here!"}</PageBodyLink></li>
                        <li>{"Wrote Python programs to report missing data in the CWE database."}</li>
                        <li>{"Conducted a security assessment of a live service and outlined recommendations for security improvements."}</li>
                        <li>{"Utilized the "} <PageBodyLink href="https://docs.aws.amazon.com/pythonsdk/">{"Boto3 AWS Python SDK"}</PageBodyLink> {" to develop network traffic, cost, and usage monitors for the "} <PageBodyLink href="https://github.com/CWE-CAPEC/REST-API-wg/blob/main/Quick%20Start.md">{"CWE REST API"}</PageBodyLink>{"."}</li>
                    </ul><br/>
                    <b>{"Tools Used: "}</b> {"Python, Node.js, Git/GitHub/GitLab, AWS, CI/CD -- GitHub Actions, Unit Testing, Integration Testing"}
                </Section>
                <Section title="AS Software -- Software Engineer Intern" subtitle="May - August 2023 | Overland Park, KS">
                    {"At "} <PageBodyLink href="https://www.as-software.com/">{"AS Software"}</PageBodyLink> {", I worked on a three-person intern team to build AS-CloudFax, a project that allows healthcare professionals to fax medical reports and images while eliminating the need for on-premises faxing hardware and server infrastructure. AS-CloudFax is cloud-native, multi-tenant, and designed to transmit PHI and PII securely."}<br/><br/>
                    {"AS-CloudFax is written with the Rust programming language using the Actix Web framework, Tokio, and SQLx with a PostgreSQL database. Through this setup, AS-CloudFax provides a reliable REST API that incorporates webhooks to deliver real-time updates. Requests to AS-CloudFax are validated for format and idempotency, and the outbox pattern is utilized to perform asynchronous actions with third-party services such as Microsoft Azure."}<br/><br/>
                    {"Using Scrum, the Agile methodology, and an automated CI/CD pipeline, we were able to build the AS-CloudFax project in ten weeks."}<br/><br/>
                    <b>{"Tools Used: "}</b> {"Rust, PostgreSQL, Docker, Git/GitHub, Azure, CI/CD, Agile/Scrum, Unit Testing, Integration Testing"}
                </Section>
            </SectionsWrapper>

            <ImageCloud>
                <Image src="/images/mitre_interns.jpeg" alt="MITRE Interns Summer 2024"></Image>
                <Image src="/images/farr-30.jpg" alt="IOR 2021"></Image>
                <Image src="/images/as_interns.jpeg" alt="AS Software Interns Summer 2023"></Image>
            </ImageCloud>

            <Footer/>
        </div>
    }
}
