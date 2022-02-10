use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AboutProps {
    pub avatar: String,
    pub name: String,
    pub profession: String,
}

#[function_component(About)]
pub fn about(
    AboutProps {
        avatar,
        name,
        profession,
    }: &AboutProps,
) -> Html {
    html! {
        <div>
            <img src={avatar.clone()}/>
            <h1>{name}</h1>
            <h2>{profession}</h2>
        </div>
    }
}
