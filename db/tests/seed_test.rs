use chrono::NaiveDate;
use db::prelude::*;
use fake::Fake;

#[tokio::test]
async fn seed() {
    use fake::faker::name::raw::*;
    use fake::locales::*;

    let list_brand = vec!["SGA".to_string(), "HNA".to_string()];

    // 2 Brand
    for brand in list_brand {
        let name: String = Name(EN).fake();
        let new_brand = BrandInput {
            name: name,
            id: brand,
        };
        let brand = svc_brand::brand_create(new_brand).await.unwrap();

        use fake::faker::address::raw::CityName;

        // 4 Campus
        for _ in 1..5 {
            let name: String = Name(EN).fake();
            let address = CityName(EN).fake();

            let new_campus = CampusInput {
                brand_id: brand.id.clone(),
                name: name,
                address: address,
                status: StatusEnum::Active,
            };
            let campus = svc_campus::campus_create(new_campus).await.unwrap();

            // 5 Class
            for _ in 1..6 {
                let name: String = Name(EN).fake();

                let new_class = ClassInput {
                    campus_id: campus.id,
                    name: name,
                    status: StatusEnum::Draft,
                    class_type: ClassTypeEnum::Main,
                    learn_year: "2023-2024".to_string(),
                };
                let class = svc_class::class_create(new_class).await.unwrap();

                // 15 Student
                for _ in 1..16 {
                    use fake::faker::address::raw::CityName;
                    use fake::faker::chrono::en::Date;
                    use fake::faker::name::raw::*;
                    use fake::locales::*;

                    let name: String = Name(EN).fake();
                    let date = Date().fake();
                    let birthplace = CityName(EN).fake();

                    let new_people = PeopleInput {
                        full_name: name,
                        birthday: date,
                        birthplace: birthplace,
                    };
                    let student = svc_student::student_create(new_people).await.unwrap();
                    let join_at = NaiveDate::parse_from_str("2023-08-28", "%Y-%m-%d").unwrap();

                    let new_st_class = StudentClassInput {
                        student_id: student.id,
                        class_id: class.id,
                        status: StatusEnum::Draft,
                        join_at: join_at,
                        leave_at: None,
                    };
                    svc_student_class::student_class_create(new_st_class)
                        .await
                        .unwrap();
                }
            }
        }
    }
}
