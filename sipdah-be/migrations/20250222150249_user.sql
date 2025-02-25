-- Add migration script here
CREATE TABLE user
(
    id           BINARY(16) PRIMARY KEY,
    email        VARCHAR(255) NOT NULL UNIQUE,
    password     VARCHAR(255) NOT NULL,
    name         VARCHAR(255) NOT NULL,
    phone_number VARCHAR(20)  NULL,
    photo_url    VARCHAR(255) NULL,
    created_at   DATETIME     NOT NULL,
    updated_at   DATETIME     NOT NULL,
    deleted_at   DATETIME     NULL
);
