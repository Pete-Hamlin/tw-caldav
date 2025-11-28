use serde::{Deserialize, Serialize};
use ureq::Agent;
use url::Url;

mod db;

// Config setup
#[derive(Serialize, Deserialize)]
pub struct TaskCaldavConfig {
    pub url: Url,
    username: String,
    password: String,
}

impl ::std::default::Default for TaskCaldavConfig {
    fn default() -> Self {
        Self {
            url: Url::parse("http://localhost").unwrap(),
            username: "user".to_string(),
            password: "password123".to_string(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent: Agent = ureq::agent();
    let cfg: TaskCaldavConfig = confy::load("tw-caldav", None)?;
    let credentials = minicaldav::Credentials::Basic(cfg.username.clone(), cfg.password.clone());
    let calendars = minicaldav::get_calendars(agent.clone(), &credentials, &cfg.url).unwrap();
    for calendar in calendars {
        println!("{:?}", calendar);
        let credentials =
            minicaldav::Credentials::Basic(cfg.username.clone(), cfg.password.clone());
        let (todos, errors) =
            minicaldav::get_todos(agent.clone(), &credentials, &calendar).unwrap();
        println!("{:?}", todos.len());
        for todo in todos {
            println!("{:?}", todo);
        }
        for error in errors {
            println!("Error: {:?}", error);
        }
    }
    Ok(())
}
