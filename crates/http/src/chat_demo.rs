pub struct ChatContext {
    pub room: domain::chat::Room,
    pub messages: Vec<crate::views::partials::ChatMessage>,
}

pub async fn load_chat_context(
    state: &crate::State,
    user_id: domain::user::Id,
) -> Result<ChatContext, crate::error::Error> {
    let chat_user_id = chat::UserId::from_uuid(*user_id.as_uuid());
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

async fn ensure_room(
    state: &crate::State,
    user_id: chat::UserId,
) -> Result<domain::chat::Room, crate::error::Error> {
    let room_name = chat::RoomName::Lobby;
    if let Some(room) = state.chat.find_room_by_name(room_name).await? {
        let _ = state
            .chat
            .join_room(
                app::chat::JoinRoom::builder()
                    .room_id(room.id)
                    .user_id(user_id)
                    .build(),
            )
            .await;
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
            let author = names
                .get(&user_id)
                .cloned()
                .unwrap_or_else(|| {
                    domain::user::Username::try_new(format!(
                        "user-{}",
                        &user_id.as_uuid().to_string()[..8]
                    ))
                    .unwrap_or_else(|_| {
                        domain::user::Username::try_new("user")
                            .expect("username")
                    })
                });
            crate::views::partials::ChatMessage::builder()
                .message_id(crate::types::Text::from(message.id.as_uuid().to_string()))
                .author(crate::types::Text::from(author.to_string()))
                .timestamp(crate::types::Text::from(format_message_time(message.created_at)))
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
use domain::chat;
