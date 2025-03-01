CREATE TABLE role
(
    id         BINARY(16) PRIMARY KEY,
    name       VARCHAR(255) NOT NULL UNIQUE,
    created_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME     NULL
);

INSERT INTO role (id, name) VALUES
                                (UUID_TO_BIN(UUID()), 'USER'),
                                (UUID_TO_BIN(UUID()), 'ADMIN'),
                                (UUID_TO_BIN(UUID()), 'SUPER_ADMIN');