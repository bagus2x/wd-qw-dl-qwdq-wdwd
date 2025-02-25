-- Add migration script here
CREATE TABLE project
(
    id         BINARY(16) PRIMARY KEY,
    author_id  BINARY(16)   NOT NULL,
    name       VARCHAR(255) NOT NULL,
    status     VARCHAR(36)  NOT NULL,
    logo_url   VARCHAR(255) NULL,
    created_at DATETIME     NOT NULL,
    updated_at DATETIME     NOT NULL,
    deleted_at DATETIME     NULL
)