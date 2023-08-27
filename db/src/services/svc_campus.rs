use diesel::*;
use diesel_async::RunQueryDsl;
use crate::{models::{Campus, CampusInput}, conn::DB};

pub async fn campus_create(model_input: CampusInput) -> anyhow::Result<Campus> {
    use crate::schema::campuses::dsl::*;
    let mut conn = DB::conn().await?;
    let campus = insert_into(campuses)
        .values(model_input)
        .returning(Campus::as_returning())
        .get_result(& mut conn)
        .await?;

    Ok(campus)
}

pub async fn campus_list_by_brand(brand_id_param: String) -> anyhow::Result<Vec<Campus>> {
    use crate::schema::campuses::dsl::*;
    
    let mut conn = DB::conn().await?;
    let list_campus = campuses
        .filter(brand_id.eq(brand_id_param))
        .select(Campus::as_select())
        .load(& mut conn)
        .await?;
    Ok(list_campus)
}