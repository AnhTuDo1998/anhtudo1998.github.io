use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ProjectInfo {
    pub title: String,
    pub category: String,
    pub link: String,
    pub techstack: String,
    pub description: Vec<String>,
}

#[derive(Properties, PartialEq)]
pub struct ProjectInfoProps {
    project_info: ProjectInfo,
}

#[function_component(Project)]
pub fn project(props: &ProjectInfoProps) -> Html {
    let project_desc: Html = props.project_info.description.iter()
    .map(|line| {
        html! {
            <li>{line}</li>
        }
    }).collect();

    html!(
        <div class="project-card">
            <h2>{format!("{}", props.project_info.title)}</h2>
            <h3>{format!("Category: {}", props.project_info.category )}</h3>
            <u>{project_desc}</u>
            <p>{"Source code for project is "}<a href={props.project_info.link.clone()}>{"here"}</a></p>
            <p class="techstack">{format!("Technologies used: {}",props.project_info.techstack)}</p>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct ProjectListProps {
    pub projects: Vec<ProjectInfo>
}

#[function_component(ProjectList)]
pub fn project_list(ProjectListProps {projects}: &ProjectListProps) -> Html {
    html! {
        <div class="project-section">
        <h1>{"Personal Projects"}</h1>
        {projects.iter().map(|proj| html! {
            <Project project_info={proj.clone()}/>
        }).collect::<Html>()}
        </div>
    }
}