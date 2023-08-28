use crate::{
    conn::DB,
    models::{Brand, BrandInput},
};
use diesel::*;
use diesel_async::RunQueryDsl;

pub async fn brand_create(model_input: BrandInput) -> anyhow::Result<Brand> {
    use crate::schema::brands::dsl::*;
    let mut conn = DB::conn().await?;
    let brand = insert_into(brands)
        .values(model_input)
        .returning(Brand::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(brand)
}
