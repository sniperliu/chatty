use regex::Regex;

pub enum Command {
    Login { username: String },
    Logout,
    To(String, String),
    Chat(String),
}

impl Command {
    pub fn parse(s: String) -> Self {
        // FIXME move the local out
        let re = Regex::new(r"^Login (.+)$").unwrap();
        if let Some(caps) = re.captures(&s.trim()) {
            return Command::Login {
                username: caps[1].to_string(),
            };
        }

        let re = Regex::new(r"^To ([^:]+):(.+)$").unwrap();
        if let Some(caps) = re.captures(&s.trim()) {
            return Command::To(caps[1].to_string(), caps[2].trim().to_string());
        }

        if s.starts_with("bye") {
            Command::Logout
        } else {
            Command::Chat(s)
        }
    }
}
