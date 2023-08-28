-- Your SQL goes here

CREATE TABLE students (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    people_id uuid NOT NULL REFERENCES peoples(id),
    status "EStatus" NOT NULL DEFAULT 'DRAFT',
    insered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX students_people_id_idx ON students(people_id);
CREATE INDEX students_status_idx ON students(status);
