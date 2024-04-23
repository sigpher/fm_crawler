-- Table for fm_crawler
CREATE TABLE foodmate(
    id SERIAL PRIMARY KEY NOT NULL,
    item_id INTEGER not null,
    title text not null,
    status text not null,
    published_at text not null,
    effective_at text not null,
    issued_by text not null
);


INSERT INTO foodmate (item_id, title, status, published_at, effective_at, issued_by) VALUES (10000, 'GB 5009.00 食品安全XX标准 永不存在', '现行有效', '2023-01-01', '2024-01-01', '国家XXXXXXXXX');


-- sqlite Table
CREATE TABLE foodmate(
    id INTEGER PRIMARY KEY NOT NULL,
    item_id INTEGER not null,
    title text not null,
    status text not null,
    published_at text not null,
    effective_at text not null,
    issued_by text not null
);