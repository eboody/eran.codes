use axum::{Form, extract::State};

use crate::{
    extract::HxRequest,
    respond::Html,
    views,
    views::users::UserVm,
    State as AppState,
};

#[derive(Debug, serde::Deserialize)]
pub struct CreateUserForm {
    pub username: String,
    pub email: String,
}

pub async fn create(
    State(_state): State<AppState>,
    HxRequest(is_hx): HxRequest,
    Form(form): Form<CreateUserForm>,
) -> Html {
    let user = UserVm {
        username: form.username,
        email: form.email,
    };

    if is_hx {
        let fragment = views::fragments::user_row::render(&user);
        return Html(fragment);
    }

    let page = views::pages::home::page(&[user]);
    Html(page)
}
