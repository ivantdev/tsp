CREATE TABLE trips (
    id SERIAL,
    user_id INTEGER NOT NULL,
    title VARCHAR(250) NULL,
    locations JSON NOT NULL,
    path JSON NOT NULL,
    distance FLOAT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_on TIMESTAMP NOT NULL,
    CONSTRAINT trips_pk PRIMARY KEY (id),
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id)
);