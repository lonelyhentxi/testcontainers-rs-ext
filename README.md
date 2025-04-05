# Testcontainers-ext

[![Crates.io](https://img.shields.io/crates/v/testcontainers-ext.svg)](https://crates.io/crates/testcontainers-ext)
[![Docs.rs](https://docs.rs/testcontainers-ext/badge.svg)](https://docs.rs/testcontainers-ext)

Testcontainers-ext is a utilities collection of extension traits for testcontainers-rs.

## Install

```bash
# or cargo add testcontainers-ext
cargo install testcontainers-ext
```

## Usage

- ImagePruneExistedLabelExt / with_prune_existed_label

```rust
use testcontainers::{core::{IntoContainerPort, WaitFor}, runners::SyncRunner, GenericImage, ImageExt};
use testcontainers_ext::ImagePruneExistedLabelExt;
use anyhow::Result;

#[tokio::test]
async fn test () -> Result<()> {
    let container = GenericImage::new("redis", "7.2.4")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .with_prune_existed_label(
            "my-project-scope",
            "redis",
            true,
            true
        ).await?
        .start()
        .await?;
    Ok(())
}
```

- ImageDefaultLogConsumerExt / with_default_log_consumer

```rust
use testcontainers::{core::{IntoContainerPort, WaitFor}, runners::SyncRunner, GenericImage, ImageExt};
use testcontainers_rs_ext::ImageDefaultLogConsumerExt;
use anyhow::Result;

#[tokio::test]
async fn test () -> Result<()> {
    let container = GenericImage::new("redis", "7.2.4")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .with_default_log_consumer()
        .start()
        .await?;
    Ok(())
}
```

## License

Licensed under

- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
