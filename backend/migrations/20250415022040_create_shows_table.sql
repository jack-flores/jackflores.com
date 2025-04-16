-- Add migration script here
CREATE TABLE shows(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    city TEXT NOT NULL,
    date DATE NOT NULL,
    poster BYTEA,
    state TEXT NOT NULL,
    ticket_link TEXT,
    venue TEXT NOT NULL
);