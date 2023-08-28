use crate::{
    conn::DB,
    models::{StudentClass, StudentClassInput},
};
use diesel::*;
use diesel_async::RunQueryDsl;

pub async fn student_class_create(model_input: StudentClassInput) -> anyhow::Result<StudentClass> {
    use crate::schema::student_class::dsl::*;
    let mut conn = DB::conn().await?;
    let st_class = insert_into(student_class)
        .values(model_input)
        .returning(StudentClass::as_returning())
        .get_result(&mut conn)
        .await?;
    Ok(st_class)
}
