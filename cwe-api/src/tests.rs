use crate::{prelude::*, BASE_URL};

fn get_client() -> Client {
    Client::new(BASE_URL)
}

#[tokio::test]
async fn can_deserialize_cwe_version() {
    get_client().get_cwe().await.unwrap();
}

#[tokio::test]
async fn can_deserialize_cwe_info() {
    get_client().get_cwe_info("all").await.unwrap();
}

#[tokio::test]
async fn can_deserialize_cwe_parents() {
    get_client().get_cwe_parents("266", None).await.unwrap();
}

#[tokio::test]
async fn can_deserialize_cwe_descendants() {
    get_client().get_cwe_descendants("137", None).await.unwrap();
}

#[tokio::test]
async fn can_deserialize_cwe_children() {
    get_client().get_cwe_children("137", None).await.unwrap();
}

#[tokio::test]
async fn can_deserialize_cwe_ancestors() {
    get_client()
        .get_cwe_ancestors("79", None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn can_deserialize_weakness_info() {
    get_client().get_cwe_weakness("all").await.unwrap();
}

#[tokio::test]
async fn can_deserialize_view_info() {
    get_client().get_cwe_view("all").await.unwrap();
}

#[tokio::test]
async fn can_deserialize_category_info() {
    get_client().get_cwe_category("all").await.unwrap();
}
