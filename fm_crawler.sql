-- Table for fm_crawler
CREATE TABLE foodmate(
    id SERIAL PRIMARY KEY NOT NULL,
    title text not null,
    status text not null,
    published_at text not null,
    effective_at text not null,
    issued_by text not null,
    link text not null
);
