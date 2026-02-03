pub struct ChatContext {
    pub room: domain::chat::Room,
    pub messages: Vec<crate::views::partials::ChatMessage>,
}

pub async fn load_chat_context(
    state: &crate::State,
    user_id: &str,
) -> Result<ChatContext, crate::error::Error> {
    let room = ensure_room(state, user_id).await?;
    let messages = state
        .chat
        .list_messages(
            app::chat::ListMessages::builder()
                .room_id(room.id.as_uuid().to_string())
                .user_id(user_id.to_string())
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
    user_id: &str,
) -> Result<domain::chat::Room, crate::error::Error> {
    let room_name = "Lobby".to_string();
    if let Some(room) = state.chat.find_room_by_name(room_name.clone()).await? {
        let _ = state
            .chat
            .join_room(
                app::chat::JoinRoom::builder()
                    .room_id(room.id.as_uuid().to_string())
                    .user_id(user_id.to_string())
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
                .created_by(user_id.to_string())
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
        let user_id = message.user_id.as_uuid().to_string();
        if names.contains_key(&user_id) {
            continue;
        }
        if let Ok(Some(user)) = state.auth.get_user(&user_id).await {
            names.insert(user_id, user.username);
        }
    }

    messages
        .iter()
        .map(|message| {
            let user_id = message.user_id.as_uuid().to_string();
            let author = names
                .get(&user_id)
                .cloned()
                .unwrap_or_else(|| format!("User {}", &user_id[..8]));
            crate::views::partials::ChatMessage::builder()
                .message_id(message.id.as_uuid().to_string())
                .author(author)
                .body(message.body.to_string())
                .status(format!("{:?}", message.status))
                .build()
        })
        .collect()
}
