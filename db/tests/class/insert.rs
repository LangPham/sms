use db::prelude::*;
use fake::Fake;
use uuid::uuid;

#[tokio::test]
async fn insert() {
    use fake::faker::name::raw::*;
    use fake::locales::*;

    let name: String = Name(EN).fake();

    let new_class = ClassInput {
        campus_id: uuid!("bed80558-49c5-40e3-9aa6-d177ca40d1c6"),
        name: name,
        status: StatusEnum::Draft,
        class_type: ClassTypeEnum::Main,
        learn_year: "2023-2024".to_string(),
    };
    let result = svc_class::class_create(new_class).await;
    assert_eq!(result.is_ok(), true);
    dbg!(result.unwrap());
}
