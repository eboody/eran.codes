use bon::Builder;

use super::{Email, Error, Username};

#[allow(unused)]
#[derive(Debug, Clone, Builder)]
pub struct NewUser {
    #[builder(with = |s: impl AsRef<str>| -> Result<_, Error> {
        Ok(Username::try_new(s.as_ref().to_owned())?)
    })]
    pub username: Username,

    #[builder(with = |s: impl AsRef<str>| -> Result<_, Error> {
        Ok(Email::try_new(s.as_ref().to_owned())?)
    })]
    pub email: Email,
}
