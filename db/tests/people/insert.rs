use db::prelude::*;
use fake::Fake;

#[tokio::test]
async fn insert() {
    use fake::faker::name::raw::*;
    use fake::faker::chrono::en::Date;
    use fake::faker::address::raw::CityName;
    use fake::locales::*;

    let name: String =  Name(EN).fake();
    let date = Date().fake();
    let birthplace = CityName(EN).fake();

    let new_people = PeopleInput {
        full_name: name,
        birthday: date,
        birthplace: birthplace
    };
    let result = svc_people::people_create(new_people).await;
    assert_eq!(result.is_ok(), true);
    dbg!(result.unwrap());
}