-- Your SQL goes here
CREATE TYPE "EClassType" AS ENUM('MAIN', 'EXTRA');

CREATE TABLE classes (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),    
    status "EStatus" NOT NULL DEFAULT 'DRAFT',
    campus_id uuid NOT NULL REFERENCES campuses(id),
    name VARCHAR NOT NULL,
    class_type "EClassType" NOT NULL DEFAULT 'MAIN',
    "learn_year" VARCHAR NOT NULL,    
    insered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX classes_campus_id_idx ON classes(campus_id);
CREATE INDEX classes_status_idx ON classes(status);
CREATE INDEX classes_class_type_idx ON classes(class_type);
CREATE INDEX classes_learn_year_idx ON classes(learn_year);