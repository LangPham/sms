use db::prelude::*;
use uuid::uuid;

#[tokio::test]
async fn select_by_class() {
    let uuid = uuid!("c34caf87-20c7-4825-b756-92e4612f5b56");
    let result = svc_student::student_list_by_class(uuid).await;
    assert_eq!(result.is_ok(), true);
    let list_student = result.unwrap();
    assert_eq!(list_student.len(), 15);
    dbg!(list_student);
}

#[tokio::test]
async fn select_by_campus() {
    let uuid = uuid!("da00e627-f465-4dff-a334-ab35eb40ed15");
    let result = svc_student::student_list_by_campus(uuid).await;
    assert_eq!(result.is_ok(), true);
    let list_student = result.unwrap();
    assert_eq!(list_student.len(), 75);
    dbg!(list_student);
}
