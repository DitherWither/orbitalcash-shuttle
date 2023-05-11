-- Add up migration script here
CREATE TABLE users (
    user_id         int         PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    display_name    text        NOT NULL,
    email           text        NOT NULL UNIQUE,
    password        text        NOT NULL,
    creation_time   timestamp   NOT NULL DEFAULT now()
);

CREATE TABLE tags (
    tag_id          int         PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    tagname         text        NOT NULL,
    user_id         int         REFERENCES users(user_id) ON DELETE CASCADE,
    UNIQUE (tagname, user_id)
);

CREATE TABLE expenses (
    expense_id      int         PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id         int         REFERENCES users(user_id) ON DELETE CASCADE,
    expense_time    timestamp   NOT NULL,
    amount          int         NOT NULL
);

CREATE TABLE expenses_tags (
    expense_id     int          REFERENCES expenses(expense_id) ON DELETE CASCADE,
    tag_id          int         REFERENCES tags(tag_id)         ON DELETE CASCADE,
    CONSTRAINT expense_tag_pkey PRIMARY KEY (expense_id, tag_id)
)