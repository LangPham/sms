-- Your SQL goes here
CREATE TABLE student_class (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id uuid NOT NULL REFERENCES students(id),
    class_id uuid NOT NULL REFERENCES classes(id),
    status "EStatus" NOT NULL DEFAULT 'DRAFT',
    join_at Date NOT NULL,
    leave_at Date,
    insered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX student_class_class_id_idx ON student_class(class_id);
CREATE INDEX student_class_student_id_idx ON student_class(student_id);
CREATE INDEX student_class_join_at_idx ON student_class(join_at);
CREATE INDEX student_class_leave_at_idx ON student_class(leave_at);
CREATE INDEX student_class_status_idx ON student_class(status);