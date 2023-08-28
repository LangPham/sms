use crate::{
    conn::DB,
    models::{Class, ClassInput},
};
use diesel::*;
use diesel_async::RunQueryDsl;

pub async fn class_create(model_input: ClassInput) -> anyhow::Result<Class> {
    use crate::schema::classes::dsl::*;
    let mut conn = DB::conn().await?;
    let class = insert_into(classes)
        .values(model_input)
        .returning(Class::as_returning())
        .get_result(&mut conn)
        .await?;
    Ok(class)
}
