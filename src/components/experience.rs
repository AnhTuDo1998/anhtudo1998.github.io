use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct JobInfo {
    pub company: String,
    pub position: String,
    pub time: String,
    pub techstack: String,
    pub responsibility: Vec<&'static str>,
}

#[derive(Properties, PartialEq)]
pub struct JobInfoProps {
    info: JobInfo,
}

#[function_component(Job)]
pub fn job(props: &JobInfoProps) -> Html {
    let job_resp_details: Html = props
        .info
        .responsibility
        .iter()
        .map(|line| {
            html! {
                <li>{line}</li>
            }
        })
        .collect();
    html!(
        <div class="experience-card">
            <h2>{format!("{} - {}",props.info.company, props.info.position)}</h2>
            <h3>{props.info.time.clone()}</h3>
            <u>{job_resp_details}</u>
            <p>{format!("Techstack: {}",props.info.techstack)}</p>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct ExperienceListProps {
    pub jobs: Vec<JobInfo>,
}

#[function_component(ExperienceList)]
pub fn experience_list(ExperienceListProps { jobs }: &ExperienceListProps) -> Html {
    html! {
        <div class="experience-section">
        <h1>{"Experience"}</h1>
        {jobs.iter().map(|job| html! {
            <Job info={job.clone()}/>
        }).collect::<Html>()}
        </div>
    }
}
