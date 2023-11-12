# dynamodb lock

Dynamodb based distributed lock implemented in pure Rust. The design is largely
inspired by
[amazon-dynamodb-lock-client](https://github.com/awslabs/amazon-dynamodb-lock-client).

It is used by the [delta-rs](https://github.com/delta-io/delta-rs) project to
implement PUT if absence semantic for concurrent S3 writes. It is considered
production ready and battle tested through the
[kafka-delta-ingest](https://github.com/delta-io/kafka-delta-ingest) project.

## Usage

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

## Deployment

Using the DynamoDb lock requires a DynamoDb table to have already been created
in AWS. The following Terraform code is an example which will create table
named "lock_example",.


```terraform
resource "aws_dynamodb_table" "oxbow_locking" {
  name         = "lock_example"
  billing_mode = "PROVISIONED"
  # Default name of the partition key hard-coded in delta-rs
  hash_key       = "key"
  read_capacity  = 10
  write_capacity = 10

  attribute {
    name = "key"
    type = "S"
  }
  # The leaseDuration is used by dynamodb-lock-rs and *must* be a Number type
  ttl {
    attribute_name = "leaseDuration"
    enabled        = true
  }
}
```

The locking code should then be configured with the environment variable
`DYNAMO_LOCK_TABLE_NAME` set to "lock_example"
