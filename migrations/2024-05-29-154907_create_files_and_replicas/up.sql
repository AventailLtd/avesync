CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    filepath VARCHAR NOT NULL,
    size INTEGER NOT NULL,
    file_date TIMESTAMP NOT NULL,
    replicas VARCHAR NOT NULL,
    to_delete BOOLEAN NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE replicas (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    ip VARCHAR NOT NULL,
    port INTEGER NOT NULL,
    connected_at TIMESTAMP NOT NULL,
    is_online BOOLEAN NOT NULL
);
