-- Add migration script here

CREATE TABLE user_role
(
    user_id     BINARY(16) NOT NULL,
    role_id     BINARY(16) NOT NULL,
    assigned_at DATETIME   NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES user (id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES role (id) ON DELETE CASCADE
);