use crate::{
    conn::DB,
    models::{People, PeopleInput, Student, StudentInput, StatusEnum},
};
use diesel::*;
use diesel_async::RunQueryDsl;

pub async fn student_create(model_input: PeopleInput) -> anyhow::Result<Student> {
    use crate::schema::{peoples::dsl::*, students::dsl::*};
    let mut conn = DB::conn().await?;
    let people = insert_into(peoples)
        .values(model_input)
        .returning(People::as_returning())
        .get_result(&mut conn)
        .await?;
    let student_input = StudentInput {
        people_id: people.id,
        status: StatusEnum::Draft,
    };

    let student = insert_into(students)
        .values(student_input)
        .returning(Student::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(student)
}
