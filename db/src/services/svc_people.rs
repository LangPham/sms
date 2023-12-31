use crate::{
    conn::DB,
    models::{People, PeopleInput},
};
use diesel::*;
use diesel_async::RunQueryDsl;

pub async fn people_create(model_input: PeopleInput) -> anyhow::Result<People> {
    use crate::schema::peoples::dsl::*;
    let mut conn = DB::conn().await?;
    let people = insert_into(peoples)
        .values(model_input)
        .returning(People::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(people)
}
