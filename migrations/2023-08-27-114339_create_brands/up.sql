-- Your SQL goes here
CREATE TABLE brands (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,    
    insered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)