use bon::Builder;

use super::Result;
use domain::user;

#[derive(Clone, Debug, Builder)]
pub struct Input {
    pub username: user::Username,
    pub email: user::Email,
}

impl Input {
    pub fn parse(username: &str, email: &str) -> Result<Self> {
        let username =
            user::Username::try_new(username).map_err(domain::user::Error::from)?;
        let email = user::Email::try_new(email).map_err(domain::user::Error::from)?;
        Ok(Self::builder().username(username).email(email).build())
    }
}
