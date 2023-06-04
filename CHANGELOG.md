# Changelog


## v0.5.0

* Remove exported `DEFAULT_MAX_RETRY_ACQUIRE_LOCK_ATTEMPTS` which is not used within the
  crate
* Introduce the `sts` feature to add `WebIdentityProvider` support for
  authentication
* Refactored `DynamoDbLockClient` instantiation to follow a builder pattern, start with `for_region` rather than `new`
