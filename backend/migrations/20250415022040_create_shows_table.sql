-- Add migration script here
CREATE TABLE shows(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    date TIMESTAMP NOT NULL,
    venue TEXT NOT NULL
);