use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GreetingProps {
    pub resume_link: String,
    pub messages: Vec<&'static str>,
}

#[function_component(Greeting)]
pub fn greeting(
    GreetingProps {
        resume_link,
        messages,
    }: &GreetingProps,
) -> Html {
    let messages: Html = {
        messages
            .iter()
            .map(|mess| {
                html! {
                    <p>{mess}</p>
                }
            })
            .collect()
    };
    html! {
        <div class="greeting">
            <h1>{"Hello World!"}</h1>
            {messages}
            <p>{"Check out my resume at "}<a href={resume_link.clone()}>{"here"}</a></p>
        </div>
    }
}
