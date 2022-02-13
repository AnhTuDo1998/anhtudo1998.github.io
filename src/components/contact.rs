use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SocialLink {
    pub platform: String,
    pub link: String,
    pub fa_string: String,
}

#[derive(PartialEq, Properties)]
pub struct ContactListProps {
    pub contacts: Vec<SocialLink>,
}

#[function_component(ContactList)]
pub fn contact_list(props: &ContactListProps) -> Html {
    let list: Html = props
        .contacts
        .iter()
        .map(|social| {
            html! {
                <li id={social.platform.clone()}>
                    <a href={social.link.clone()}><i class={social.fa_string.clone()}/></a>
                </li>
            }
        })
        .collect();

    html! {
        <u class="contacts">{list}</u>
    }
}
