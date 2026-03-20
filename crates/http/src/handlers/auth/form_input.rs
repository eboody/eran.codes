use secrecy::SecretString;

use super::{LoginForm, NextPath, RegisterForm};

#[derive(Clone, Debug)]
pub(super) struct LoginInput {
    pub next: Option<String>,
    pub credentials: app::auth::Credentials,
}

impl TryFrom<LoginForm> for LoginInput {
    type Error = crate::Error;

    fn try_from(form: LoginForm) -> Result<Self, Self::Error> {
        let next = NextPath::sanitize(form.next);
        let email = parse_email(form.email)?;
        let credentials = app::auth::Credentials::builder()
            .email(email)
            .password(SecretString::new(form.password.to_string().into()))
            .build();

        Ok(Self { next, credentials })
    }
}

#[derive(Clone, Debug)]
pub(super) struct RegisterInput {
    pub next: Option<String>,
    pub command: app::user::Register,
    pub credentials: app::auth::Credentials,
}

impl TryFrom<RegisterForm> for RegisterInput {
    type Error = crate::Error;

    fn try_from(form: RegisterForm) -> Result<Self, Self::Error> {
        let next = NextPath::sanitize(form.next);
        let parsed =
            app::user::Input::parse(&form.username.to_string(), &form.email.to_string())
                .map_err(crate::Error::from)?;
        let password = SecretString::new(form.password.to_string().into());

        let command = app::user::Register::builder()
            .username(parsed.username)
            .email(parsed.email.clone())
            .password(password.clone())
            .build();
        let credentials = app::auth::Credentials::builder()
            .email(parsed.email)
            .password(password)
            .build();

        Ok(Self {
            next,
            command,
            credentials,
        })
    }
}

fn parse_email(value: crate::types::Text) -> crate::Result<domain::user::Email> {
    domain::user::Email::try_new(value.to_string())
        .map_err(domain::user::Error::from)
        .map_err(app::user::Error::from)
        .map_err(crate::Error::from)
}
