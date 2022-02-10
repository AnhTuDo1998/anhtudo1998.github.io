use yew::prelude::*;

mod components;
use components::about::About;
use components::contact::{ContactList, SocialLink};

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

    html! {
        <>
            <header>
            <div class="root">
                <div class="sidebar">
                    <About avatar={avatar_path} name={name} profession={profession}/>
                    <ContactList contacts={socials}/>
                </div>
                <div class="content">
                    <p>{"Example test here here!"}</p>
                </div>
            </div>
            </header>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
