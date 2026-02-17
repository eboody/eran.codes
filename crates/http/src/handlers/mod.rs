mod auth;
mod demo;
mod pages;
mod sse;

pub use auth::{login, login_form, logout, protected, register, register_form};
pub use demo::{
    auth_status_partial, boundary_check_partial, chat_page, db_check_partial,
    moderate_message, moderation_page, ping_partial, post_chat_message,
    post_demo_chat_message, request_meta_partial, session_status_partial,
};
pub use pages::{error_test, health, home};
pub use sse::{events, surreal_message_cancel, surreal_message_guarded};
