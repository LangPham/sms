use chrono::NaiveDate;
use db::prelude::*;
use uuid::uuid;

#[tokio::test]
async fn insert() {
    let join_at = NaiveDate::parse_from_str("2023-08-28", "%Y-%m-%d").unwrap();

    let new_st_class = StudentClassInput {
        student_id: uuid!("2cb2671b-77cd-4994-8393-c926cf0b4b80"),
        class_id: uuid!("2371415b-0ad9-4021-8c0d-9b3031c549c3"),
        status: StatusEnum::Draft,
        join_at: join_at,
        leave_at: None,
    };

    let result = svc_student_class::student_class_create(new_st_class).await;
    assert_eq!(result.is_ok(), true);
    dbg!(result.unwrap());
}
