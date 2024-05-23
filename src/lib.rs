pub use reqwest::blocking::Client;
use log::{info, error};
use gargoyle::{Action, Monitor};

/// The `WebAvailability` struct represents a monitor that checks the availability of 
/// a web service.
///
///  # Example
///
///  ```
///  # use std::time::Duration;
///  # use std::thread::sleep;
///  # use gargoyle::Notify;
///  # struct Sink;
///  # impl Notify for Sink {
///  #    fn send(&self, _msg: &str, _diagnostic: Option<String>) -> Result<(), String> {
///  #        Ok(())
///  #    }
///  # }
///  use gargoyle::Schedule;
///  use gargoyle_web_monitor::WebAvailability;
///  let url = "http://example.com";
///  let web_monitor = WebAvailability::with_user_agent(url, "Gargoyle/0.1 my_contact_info@example.com").unwrap();
///  let sink_notifier = Sink;
///  let mut schedule = Schedule::default();
///  schedule.add(
///      &format!("The Gargoyle has detected that {url} has gone down"),
///      &format!("The Gargoyle has detected that {url} has recovered"),
///      Duration::from_secs(60),
///      &web_monitor,
///      &sink_notifier,
/// );
///
/// loop {
///    schedule.run();
///    sleep(Duration::from_millis(100));
/// }
/// ```
pub struct WebAvailability {
    pub url: String,
    web_client: Client,
}

impl WebAvailability {
    pub fn new(url: &str) -> Result<Self, String> {
        let web_client = Client::builder()
            .user_agent("Gargoyle/0.1")
            .build();
        if let Err(e) = web_client {
            return Err(format!("{e}"));
        }
        let web_client = web_client.unwrap();
        Ok(Self {
            url: url.to_string(),
            web_client,
        })
    }

    pub fn with_user_agent(url: &str, user_agent: &str) -> Result<Self, String> {
        let web_client = Client::builder()
            .user_agent(user_agent)
            .build();
        if let Err(e) = web_client {
            return Err(format!("{e}"));
        }
        let web_client = web_client.unwrap();
        Ok(Self {
            url: url.to_string(),
            web_client,
        })
    }

    pub fn with_client(url: &str, web_client: Client) -> Self {
        Self {
            url: url.to_string(),
            web_client,
        }
    }
}

/// Check the availability of a web service.
impl Monitor for WebAvailability {
    fn check(&mut self) -> Action {
        match self.web_client.get(&self.url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    info!("{} is up", self.url);
                    Action::Nothing
                } else {
                    info!("{} is down", self.url);
                    error!("Failed to get {} - {}", self.url, response.status());
                    Action::Notify {
                        diagnostic: Some(format!("Failed to get {} - {}", self.url, response.status())),
                    }
                }
            }
            Err(_) => {
                info!("{} is down", self.url);
                error!("Failed to connect to {}", self.url);
                Action::Notify {
                    diagnostic: Some(format!("Failed to connect to {}", self.url)),
                }
            }
        }
    }
}

