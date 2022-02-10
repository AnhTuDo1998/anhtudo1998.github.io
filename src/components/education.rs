use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EducationProps {
    pub school: String,
    pub degree: String,
    pub time: String,
    pub info_list: Vec<&'static str>,
    //pub logo: String,
}

#[function_component(Education)]
pub fn education(
    EducationProps {
        school,
        degree,
        time,
        info_list,
    }: &EducationProps,
) -> Html {
    let education_details: Html = info_list
        .iter()
        .map(|info| {
            html! {
                <li>{info}</li>
            }
        })
        .collect();

    html!(
        <div class="education-info">
            <h1>{"Education"}</h1>
            <h2>{school}</h2>
            <h3>{format!("{}({})", degree, time)}</h3>
            <u class="education-details">
                {education_details}
            </u>
        </div>
    )
}
