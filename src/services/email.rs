use mailgun_rs::{Mailgun, EmailAddress, Message, SendResponse};
use std::error::Error;

pub struct EmailService<'a> {
    pub to: &'a str,
    pub subject: String,
    pub html: String,
    #[cfg(debug_assertions)]
    pub force_in_debug: bool,
}

impl<'a> EmailService<'a> {
    pub fn new(to: &'a str, subject: String, html: String) -> Self {
        EmailService {
            to,
            subject,
            html,
            #[cfg(debug_assertions)]
            force_in_debug: false
        }
    }

    #[cfg(debug_assertions)]
    pub fn send(&self) -> Result<SendResponse, Box<dyn Error>> {
        if self.force_in_debug {
            self._send()
        } else {
            Ok(SendResponse {
                message: String::from("Nothing here. We are in debug"),
                id: String::from("A fake id")
            })
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn send(&self) -> Result<SendResponse, Box<dyn Error>> {
        self._send()
    }

    fn _send(&self) -> Result<SendResponse, Box<dyn Error>> {
        let message = Message {
            to: vec![EmailAddress::address(&self.to)],
            subject: self.subject.clone(), 
            html: self.html.clone(),
            ..Default::default()
        };

        let client = Mailgun {
            api_key: std::env::var("MAILGUN_KEY")
                .expect("MAILGUN_KEY must be set"),
            domain: std::env::var("MAILGUN_DOMAIN")
                .expect("MAILGUN_DOMAIN must be set"),
            message: message
        };
        
        let sender = EmailAddress::name_address(
            "no-reply",
            &std::env::var("MAILGUN_MAIL_ADDRESS").expect("MAILGUN_MAIL_ADDRESS must be set"));

        let response = client.send(&sender)?;
        Ok(response)
    }
}