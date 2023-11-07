dynamodb lock
=============

Dynamodb based distributed lock implemented in pure Rust. The design is largely
inspired by
[amazon-dynamodb-lock-client](https://github.com/awslabs/amazon-dynamodb-lock-client).

It is used by the [delta-rs](https://github.com/delta-io/delta-rs) project to
implement PUT if absence semantic for concurrent S3 writes. It is considered
production ready and battle tested through the
[kafka-delta-ingest](https://github.com/delta-io/kafka-delta-ingest) project.

Usage
-----

```rust
let region = dynamodb_lock::Region::default();
// This will use the default options
let lock_client = dynamodb_lock::DynamoDbLockClient::for_region(region);

let lock_data = "Moe";
let lock = lock_client.try_acquire_lock(Some(lock_data)).await?.unwrap();

if lock.acquired_expired_lock {
    // error handling when acquired an expired lock
}

// do stuff in the critical region

lock_client.release_lock(&lock).await?;
```

For real world example, see
https://github.com/delta-io/delta-rs/blob/main/rust/src/storage/s3/mod.rs.
