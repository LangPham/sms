use db::prelude::*;
use fake::Fake;

#[tokio::test]
async fn insert() {
    use fake::faker::name::raw::*;
    use fake::locales::*;

    let name: String = Name(EN).fake();

    let new_brand = BrandInput {
        name: name,
        id: "SGA".to_string(),
    };
    let result = svc_brand::brand_create(new_brand).await;
    assert_eq!(result.is_ok(), true);
    dbg!(result.unwrap());
}
