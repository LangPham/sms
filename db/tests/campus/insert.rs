use db::prelude::*;
use fake::Fake;

#[tokio::test]
async fn insert() {
    use fake::faker::address::raw::CityName;
    use fake::faker::name::raw::*;
    use fake::locales::*;

    let name: String = Name(EN).fake();
    let address = CityName(EN).fake();

    let new_campus = CampusInput {
        brand_id: "SGA".to_string(),
        name: name,
        address: address,
        status: StatusEnum::Draft,
    };
    let result = svc_campus::campus_create(new_campus).await;
    assert_eq!(result.is_ok(), true);
    dbg!(result.unwrap());
}
