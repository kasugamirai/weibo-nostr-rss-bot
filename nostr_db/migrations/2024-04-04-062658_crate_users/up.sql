CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    avatar VARCHAR  NULL,
    publickey VARCHAR NOT NULL,
    privatekey VARCHAR NOT NULL,
    u_id INTEGER NOT NULL
)
