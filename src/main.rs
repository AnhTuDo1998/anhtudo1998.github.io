use yew::prelude::*;

mod components;
use components::about::About;

#[function_component(App)]
fn app() -> Html {
    // data
    let avatar_path = "";
    let name = "Do Anh Tu";
    let profession = "Embedded Software Engineer";

    html! {
        <>
            <div className="wrapper">
                <div className="sidebar">
                    <About avatar={avatar_path} name={name} profession={profession} />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}