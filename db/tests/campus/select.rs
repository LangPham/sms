use db::prelude::*;

#[tokio::test]
async fn select_by_brand() {
    
    let brand_id_param = "SGA".to_string();
    

    
    let result = svc_campus::campus_list_by_brand(brand_id_param).await;
    assert_eq!(result.is_ok(), true);
    dbg!(result.unwrap());
}
