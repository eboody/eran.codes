moddef::moddef!(
    mod {
        chat,
        chat_moderation,
        home,
        lab,
        login,
        open_source,
        portfolio_shell,
        protected,
        register,
        work,
        work_case
    }
);

pub use chat_moderation::ChatModeration;
pub use home::Home;
pub use lab::Lab;
pub use login::Login;
pub use open_source::OpenSource;
pub use protected::Protected;
pub use register::Register;
pub use work::Work;
pub use work_case::WorkCase;
