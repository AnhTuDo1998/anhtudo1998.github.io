use yew::{function_component, Properties, html, Html};

#[derive(Properties, PartialEq)]
pub struct AboutProps {
    pub avatar: String,
    pub name: String,
    pub profession: String,
}

#[function_component(About)]
pub fn about(AboutProps {avatar, name, profession}: &AboutProps) -> Html {
    html! {
        <div>
            <div>
                <img src={avatar.clone()}/>
            </div>
            <div>
                <h1>{name}</h1>
                <h2>{profession}</h2>
            </div>
        </div>
    }
}