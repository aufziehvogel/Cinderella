use lettre_email::Email;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SmtpClient, Transport};
use lettre::smtp::ConnectionReuseParameters;

use crate::config;

pub fn send_mail(text: &str, config: &config::Email) {
    let email = Email::builder()
        .to(config.to.to_string())
        .from(config.from.to_string())
        .subject("Build failed")
        .text(text)
        .build()
        .unwrap();

    let mut mailer = SmtpClient::new_simple(&config.server).unwrap()
        .credentials(Credentials::new(
            config.user.to_string(),
            config.password.to_string()))
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).transport();

    if let Err(_err) = mailer.send(email.into()) {
        // TODO: Add logging here
        panic!("Sending of mail failed");
    }
}
