DROP Table if EXISTS users, notifications, messages, groups, groups_users;
DROP TYPE msg_type;
CREATE TYPE msg_type AS ENUM ('text', 'video', 'photo');


