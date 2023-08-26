-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE peoples (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    full_name VARCHAR NOT NULL,
    birthday DATE NOT NULL,
    birthplace VARCHAR NOT NULL,
    insered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)