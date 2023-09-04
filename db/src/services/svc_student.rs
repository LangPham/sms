use crate::{
    conn::DB,
    models::{Campus, Class, People, PeopleInput, StatusEnum, Student, StudentInput},
};
use diesel::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

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

pub async fn student_list_by_class(param_class_id: Uuid) -> anyhow::Result<Vec<(Class, Student)>> {
    use crate::schema;
    let mut conn = DB::conn().await?;
    let list_student = schema::student_class::table
        .inner_join(schema::classes::table)
        .inner_join(schema::students::table)
        .filter(schema::student_class::class_id.eq(param_class_id))
        .select((Class::as_select(), Student::as_select()))
        .load::<(Class, Student)>(&mut conn)
        .await?;

    Ok(list_student)
}

pub async fn student_list_by_campus(
    param_campus_id: Uuid,
) -> anyhow::Result<Vec<(Campus, Student)>> {
    use crate::schema;
    let mut conn = DB::conn().await?;
    let list_student = schema::campuses::table
        .inner_join(
            schema::classes::table
                .inner_join(schema::student_class::table.inner_join(schema::students::table)),
        )
        .filter(schema::campuses::id.eq(param_campus_id))
        .select((Campus::as_select(), Student::as_select()))
        .load::<(Campus, Student)>(&mut conn)
        .await?;

    Ok(list_student)
}
