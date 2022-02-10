use yew::prelude::*;

mod components;
use components::about::About;
use components::contact::{ContactList, SocialLink};
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
        },
        SocialLink {
            platform: "github".to_string(),
            link: "https://www.github.com/AnhTuDo1998".to_string(),
        },
        SocialLink {
            platform: "mail".to_string(),
            link: "mailto:anhtu.do1998@gmail.com".to_string(),
        },
    ];
    let greeting_msg = vec!["I am Anh Tu, a young IT professional working in Singapore. I graduated from SCSE NTU Singapore in 2021, studying Computer Engineering. I am, currently an Embedded Software Engineer at Thales (DIS) Singapore, focusing on developing operating systems for embedded Secure Elements.",
    "During free time, I love to explore new technologies to broaden my horizons. I am currently learning and building various projects in Rustlang, from web to blockchain to embedded projects."];
    // TODO
    let resume_link = "thisisalink";
    
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
                </div>
            </div>
            </header>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
