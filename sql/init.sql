DROP Table if EXISTS friends, users, notifications, messages, groups, groups_users;
DROP TYPE msg_type;
CREATE TYPE msg_type AS ENUM ('text', 'video', 'photo');


CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL, 
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY NOT NULL,
    sender SERIAL REFERENCES users(id),
    receiver SERIAL REFERENCES users(id),
    encoded MSG_TYPE, 
    content TEXT,
    is_seen BOOLEAN NOT NULL DEFAULT false
);

CREATE table friends (
    user_id SERIAL REFERENCES users(id),
    room_id SERIAL UNIQUE NOT NULL,
    friend SERIAL REFERENCES users(id)
);

CREATE TABLE notifications (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id SERIAL REFERENCES users(id),
    content TEXT NOT NULL,
    is_seen BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE groups (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    owner SERIAL REFERENCES users(id)
);

CREATE TABLE groups_users (
    user_id SERIAL REFERENCES users(id),
    group_id SERIAL REFERENCES groups(id)
);
