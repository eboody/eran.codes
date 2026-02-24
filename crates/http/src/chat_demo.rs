use domain::chat;

const DEMO_USER_EMAIL: &str = "demo.bot@example.com";
const DEMO_USER_NAME: &str = "Demo Bot";

pub struct ChatContext {
    pub room: domain::chat::Room,
    pub messages: Vec<crate::views::partials::ChatMessage>,
}

pub async fn load_chat_context(
    state: &crate::State,
    user_id: Option<domain::user::Id>,
) -> Result<ChatContext, crate::error::Error> {
    let viewer = match user_id {
        Some(user_id) => user_id,
        None => ensure_demo_user(state).await?.id,
    };
    let chat_user_id = chat::UserId::from_uuid(*viewer.as_uuid());
    let room = ensure_room(state, chat_user_id).await?;
    let messages = state
        .chat
        .list_messages(
            app::chat::ListMessages::builder()
                .room_id(room.id)
                .user_id(chat_user_id)
                .build(),
        )
        .await?;
    let message_views = to_message_views(state, &messages).await;

    Ok(ChatContext {
        room,
        messages: message_views,
    })
}

pub async fn ensure_demo_user(
    state: &crate::State,
) -> Result<domain::user::User, crate::error::Error> {
    let demo_email = domain::user::Email::try_new(DEMO_USER_EMAIL)
        .map_err(|_| crate::error::Error::Internal)?;
    let demo_username = domain::user::Username::try_new(DEMO_USER_NAME)
        .map_err(|_| crate::error::Error::Internal)?;
    if let Some(user) = state.user.find_by_email(demo_email.clone()).await? {
        return Ok(user);
    }

    let password = secrecy::SecretString::new(uuid::Uuid::new_v4().to_string().into());
    match state
        .user
        .register_user(
            app::user::RegisterUser::builder()
                .username(demo_username)
                .email(demo_email.clone())
                .password(password)
                .build(),
        )
        .await
    {
        Ok(_) | Err(app::user::Error::EmailTaken) => {}
        Err(error) => return Err(error.into()),
    }

    state
        .user
        .find_by_email(demo_email)
        .await?
        .ok_or(crate::error::Error::Internal)
}

async fn ensure_room(
    state: &crate::State,
    user_id: chat::UserId,
) -> Result<domain::chat::Room, crate::error::Error> {
    let room_name = chat::RoomName::Lobby;
    if let Some(room) = state.chat.find_room_by_name(room_name).await? {
        state
            .chat
            .join_room(
                app::chat::JoinRoom::builder()
                    .room_id(room.id)
                    .user_id(user_id)
                    .build(),
            )
            .await?;
        return Ok(room);
    }

    let room = state
        .chat
        .create_room(
            app::chat::CreateRoom::builder()
                .name(room_name)
                .created_by(user_id)
                .build(),
        )
        .await?;
    Ok(room)
}

async fn to_message_views(
    state: &crate::State,
    messages: &[domain::chat::Message],
) -> Vec<crate::views::partials::ChatMessage> {
    let mut names = std::collections::HashMap::new();
    for message in messages {
        let user_id = domain::user::Id::from_uuid(*message.user_id.as_uuid());
        if names.contains_key(&user_id) {
            continue;
        }
        if let Ok(Some(user)) = state.auth.get_user(&user_id).await {
            names.insert(user_id, user.username);
        }
    }

    messages
        .iter()
        .rev()
        .map(|message| {
            let user_id = domain::user::Id::from_uuid(*message.user_id.as_uuid());
            let author = names.get(&user_id).cloned().unwrap_or_else(|| {
                domain::user::Username::try_new(format!(
                    "user-{}",
                    &user_id.as_uuid().to_string()[..8]
                ))
                .unwrap_or_else(|_| {
                    domain::user::Username::try_new("user").expect("username")
                })
            });
            crate::views::partials::ChatMessage::builder()
                .message_id(crate::types::Text::from(message.id.as_uuid().to_string()))
                .author(crate::types::Text::from(author.to_string()))
                .timestamp(crate::types::Text::from(format_message_time(
                    message.created_at,
                )))
                .body(crate::types::Text::from(message.body.to_string()))
                .status(crate::types::Text::from(format!("{:?}", message.status)))
                .build()
        })
        .collect()
}

pub fn format_message_time(value: std::time::SystemTime) -> String {
    let time = time::OffsetDateTime::from(value);
    let format = time::format_description::parse(
        "[year]-[month]-[day] [hour repr:24 padding:zero]:[minute padding:zero]",
    )
    .unwrap_or_else(|_| Vec::new());
    time.format(&format).unwrap_or_else(|_| "--:--".to_string())
}
