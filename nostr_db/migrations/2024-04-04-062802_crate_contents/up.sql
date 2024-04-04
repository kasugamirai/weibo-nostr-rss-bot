CREATE TABLE contents (
    id SERIAL PRIMARY KEY,
    author VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    link VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    user_id INTEGER REFERENCES users(id)  NOT NULL
)
