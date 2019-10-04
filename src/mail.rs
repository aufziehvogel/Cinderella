use lettre_email::Email;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SmtpClient, Transport};
use lettre::smtp::ConnectionReuseParameters;

use crate::config;

pub trait Mailer {
    fn send_mail(&self, name: &str, text: &str);
}

struct NullMailer;

struct SmtpMailer {
    pub server: String,
    pub user: String,
    pub password: String,
    pub from: String,
    pub to: String,
}

impl SmtpMailer {
    pub fn from_config(config: &config::Email) -> SmtpMailer {
        SmtpMailer {
            server: config.server.clone(),
            user: config.user.clone(),
            password: config.password.clone(),
            from: config.from.clone(),
            to: config.to.clone(),
        }
    }
}

impl Mailer for NullMailer {
    fn send_mail(&self, _name: &str, _text: &str) {

    }
}

impl Mailer for SmtpMailer {
    fn send_mail(&self, name: &str, text: &str) {
        let email = Email::builder()
            .to(self.to.to_string())
            .from(self.from.to_string())
            .subject(format!("Build failed: {}", name))
            .text(text)
            .build()
            .unwrap();

        let mut mailer = SmtpClient::new_simple(&self.server).unwrap()
            .credentials(Credentials::new(
                self.user.to_string(),
                self.password.to_string()))
            .smtp_utf8(true)
            .authentication_mechanism(Mechanism::Plain)
            .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).transport();

        if let Err(_err) = mailer.send(email.into()) {
            // TODO: Add logging here
            panic!("Sending of mail failed");
        }
    }
}

pub fn build_mailer(config: &Option<config::Email>) -> Box<dyn Mailer> {
    match config {
        None => Box::new(NullMailer),
        Some(config) => Box::new(SmtpMailer::from_config(&config)),
    }
}
