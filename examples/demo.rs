use dynamodb_lock::*;

#[tokio::main]
async fn main() -> Result<(), DynamoError> {
    pretty_env_logger::init();
    let region = Region::default();
    // This will use the default options
    let lock_client = DynamoDbLockClient::for_region(region);

    let lock_data = "Moe";
    let lock = lock_client
        .try_acquire_lock(Some(lock_data))
        .await?
        .unwrap();

    if lock.acquired_expired_lock {
        // error handling when acquired an expired lock
    }

    // do stuff in the critical region
    lock_client.release_lock(&lock).await?;
    Ok(())
}
