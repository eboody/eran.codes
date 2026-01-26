ALTER TABLE chat_rooms
    ADD CONSTRAINT chat_rooms_name_key UNIQUE (name);
