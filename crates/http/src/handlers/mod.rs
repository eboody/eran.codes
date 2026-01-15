mod auth;
mod pages;
mod partials;
mod sse;

pub use auth::{login, login_form, logout, protected, register, register_form};
pub use pages::{error_test, health, home};
pub use partials::{
    auth_status_partial, boundary_check_partial, db_check_partial, ping_partial,
    request_meta_partial, session_status_partial,
};
pub use sse::{events, surreal_message_cancel, surreal_message_guarded};
