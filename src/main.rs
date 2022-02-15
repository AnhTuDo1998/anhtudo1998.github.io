use yew::prelude::*;

mod components;
use components::about::About;
use components::contact::{ContactList, SocialLink};
use components::education::Education;
use components::experience::{ExperienceList, JobInfo};
use components::greeting::Greeting;

#[function_component(App)]
fn app() -> Html {
    // data
    let avatar_path = "static/avatar2.jpg";
    let name = "Do Anh Tu";
    let profession = "Embedded Software Engineer";
    let socials = vec![
        SocialLink {
            platform: "linkedin".to_string(),
            link: "https://www.linkedin.com/in/anhtudo1998
        "
            .to_string(),
            fa_string: "fa-brands fa-linkedin".to_string(),
        },
        SocialLink {
            platform: "github".to_string(),
            link: "https://www.github.com/AnhTuDo1998".to_string(),
            fa_string: "fa-brands fa-github-square".to_string(),
        },
        SocialLink {
            platform: "mail".to_string(),
            link: "mailto:anhtu.do1998@gmail.com".to_string(),
            fa_string: "fa-solid fa-envelope".to_string(),
        },
    ];
    let greeting_msg = vec!["I am Anh Tu, a young IT professional working in Singapore. I graduated from SCSE NTU Singapore in 2021, studying Computer Engineering. I am, currently an Embedded Software Engineer at Thales (DIS) Singapore, focusing on developing operating systems for embedded Secure Elements.",
    "During free time, I love to explore new technologies to broaden my horizons. I am currently learning and building various projects in Rustlang, from web to blockchain to embedded projects."];
    // TODO
    let resume_link = "static/resume.pdf";
    let school = "Nanyang Technological University";
    let degree = "B.Eng - Computer Engineering";
    let time = "Jul 2017 - Jun 2021";
    let education_details = vec![
        "GPA: 4.76/5.00",
        "Honours (Highest Distinction)",
        "Elective Focus in Artificial Intelligence and Data Science",
        "Dean's List (AY 2018-2019 & AY 2020-2021)",
        "NTU CAC Lindy Hop - Chairperson (AY18/19), Dance Captain (AY19/20)",
        "NTU SCSE Club - Academic Subcommittee Member (AY19/20)",
    ];

    let experiences = vec![
        JobInfo {
            company: "Thales (DIS) Singapore".to_string(),
            position: "Embedded Software Engineer".to_string(),
            time: "Sep 2021 - Current".to_string(),
            techstack: "C, Java, Keil uVision, Bitbucket, Jira, git".to_string(),
            responsibility: vec!["Analyze potential C language vulnerabilities and implements fixes for Thales's propietary card OS.",
            "Monitor code coverage of low-level OS module for use in CC certification.", "Devise OS test plan for 2 products of embedded Secure Element."],
        },
        JobInfo {
            company: "Nanyang Technological University".to_string(),
            position: "Research Assistant".to_string(),
            time: "May 2021 - Jul 2021".to_string(),
            techstack: "C, C++, Cmake, Linux".to_string(),
            responsibility: vec!["Build embedded Linux images for Xilinx ZCU102 FPGA based on Yocto Project and Xilinx's Petalinux for evaluations of SLAM algorithm.",
            "Migrate SLAM algorithms from x64 to aarch64 system.", "Set up CMake and cross-compilation toolchain for building dependencies and libraries of system from source.",
            "Integrate SLAM algoritms to the built images and evaluate the system performance at runtime."],
        },
        JobInfo {
            company: "Thermo Fisher Scientific".to_string(),
            position: "Firmware Engineer Intern".to_string(),
            time: "Jan 2020 - Jul 2020".to_string(),
            techstack: "C, Python, Linux , Keil uVision, Bitbucket, Jira, git".to_string(),
            responsibility: vec!["Design internal Python-based tool for communicating and sniffing CAN-based protocols.",
            "Design a rotating 4 electro-magnets system with ARM CMSIS RTOS for use by internal scientists to carry out biomolecular experiment.",
            "Set up Amazon FreeRTOS on NXP iMXRT1020EVK MCUs and tested its integration to AWS IoT
            services as potential alternative for MQTT client in production.", "Design a low-level library for playing tunes on piezo buzzers to provide notification sounds to
            users in company’s upcoming products."],
        },
        JobInfo {
            company: "Konini Vending Automation".to_string(),
            position: "Hardware Engineer".to_string(),
            time: "May 2019 - Jul 2019".to_string(),
            techstack: "C, Java, Keil uVision, Bitbucket, Jira, git".to_string(),
            responsibility: vec!["Collect food’s images and labelled them to train object detection models which are then
            integrated to the company’s new products.","Design testing plans and conducted black box testing on the company’s unmanned fridges,
            unmanned lockers and unmanned food canteen for quality assurance before deployments."],
        },
    ];

    // HTML skeleton
    html! {
        <>
            <header>
            <div class="root">
                <div class="sidebar">
                    <About avatar={avatar_path} name={name} profession={profession}/>
                    <ContactList contacts={socials}/>
                </div>
                <div class="content">
                    <Greeting {resume_link} messages={greeting_msg}/>
                    <ExperienceList jobs={experiences}/>
                    <Education {school} {degree} {time} info_list={education_details}/>
                </div>
            </div>
            </header>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}