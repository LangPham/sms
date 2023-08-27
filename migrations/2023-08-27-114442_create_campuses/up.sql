-- Your SQL goes here

CREATE TYPE "EStatus" AS ENUM('DRAFT', 'ACTIVE', 'DELETED', 'ARCHIVE');

CREATE TABLE campuses (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    brand_id VARCHAR NOT NULL REFERENCES brands(id),
    "name" VARCHAR NOT NULL,    
    address VARCHAR NOT NULL,
    status "EStatus" NOT NULL DEFAULT 'DRAFT',
    insered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX campuses_brand_id_idx ON campuses(brand_id);
CREATE INDEX campuses_status_idx ON campuses(status);