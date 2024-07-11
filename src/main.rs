use futures_util::{stream, StreamExt};
use object_store::{aws::AmazonS3Builder, ClientOptions, ObjectStore};
use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let builder = AmazonS3Builder::new()
        .with_region(std::env::var("AWS_DEFAULT_REGION").unwrap())
        .with_bucket_name(std::env::var("AWS_BUCKET").unwrap())
        .with_access_key_id(std::env::var("AWS_ACCESS_KEY_ID").unwrap())
        .with_secret_access_key(std::env::var("AWS_SECRET_ACCESS_KEY").unwrap())
        .with_client_options(ClientOptions::new().with_timeout(Duration::from_secs(60)))
        .with_imdsv1_fallback();

    let object_store = Arc::new(builder.build().expect("valid object store"));

    let valid_object_store_path =
        object_store::path::Path::from(std::env::var("OBJECT_STORE_PATH").unwrap());

    let num_concurrent_tasks = std::env::var("NUM_CONCURRENT_TASKS")
        .unwrap()
        .parse()
        .unwrap();

    let mut handles = stream::iter(0..50)
        .map(|i| {
            let object_store = Arc::clone(&object_store);
            let path = valid_object_store_path.clone();

            tokio::task::spawn(async move {
                println!("starting {i}");

                let res = match object_store.get(&path).await {
                    Ok(get_result) => get_result.bytes().await,
                    Err(e) => Err(e),
                };
                (i, res)
            })
        })
        .buffer_unordered(num_concurrent_tasks);

    while let Some(task) = handles.next().await {
        let (i, result) = task.expect("task shouldnt have panicked");
        println!("result for task {i}: {:?}", result.map(|bytes| bytes.len()));
    }
}
