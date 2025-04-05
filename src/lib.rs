#![crate_name = "testcontainers_ext"]

use bollard::container::ListContainersOptions;
use std::future::Future;
use testcontainers::{ContainerRequest, Image, ImageExt, TestcontainersError};

pub trait ImagePruneExistedLabelExt<I>: Sized + ImageExt<I> + Send
where
    I: Image,
{
    /// Given a scope, a container label, a prune flag, and a force flag,
    /// this method will prune the container if the prune flag is true.
    ///
    /// Example:
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    /// use testcontainers::{core::{IntoContainerPort, WaitFor}, runners::AsyncRunner, GenericImage, ImageExt};
    /// use testcontainers_ext::ImagePruneExistedLabelExt;
    /// use anyhow::Result;
    ///
    /// async fn test () -> Result<()> {
    ///   let container = GenericImage::new("redis", "7.2.4")
    ///         .with_exposed_port(6379.tcp())
    ///         .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
    ///         .with_prune_existed_label("my-project-scope", "redis", true, true).await?
    ///         .start()
    ///         .await?;
    ///    Ok(())
    /// }
    ///
    /// Runtime::new().unwrap().block_on(test()).unwrap();
    /// ```
    ///
    fn with_prune_existed_label(
        self,
        scope: &str,
        container_label: &str,
        prune: bool,
        force: bool,
    ) -> impl Future<Output = Result<ContainerRequest<I>, TestcontainersError>> + Send {
        use std::collections::HashMap;

        use bollard::container::PruneContainersOptions;
        use testcontainers::core::client::docker_client_instance;

        let testcontainers_project_key = format!("{scope}.testcontainers.scope");
        let testcontainers_container_key = format!("{scope}.testcontainers.container");
        let testcontainers_prune_key = format!("{scope}.testcontainers.prune");

        async move {
            if prune {
                let client = docker_client_instance().await?;

                let mut filters = HashMap::<String, Vec<String>>::new();

                filters.insert(
                    String::from("label"),
                    vec![
                        format!("{testcontainers_prune_key}=true"),
                        format!("{}={}", testcontainers_project_key, scope),
                        format!("{}={}", testcontainers_container_key, container_label),
                    ],
                );

                if force {
                    let result = client
                        .list_containers(Some(ListContainersOptions {
                            all: false,
                            filters: filters.clone(),
                            ..Default::default()
                        }))
                        .await
                        .map_err(|err| TestcontainersError::Other(Box::new(err)))?;

                    let remove_containers = result
                        .iter()
                        .filter(|c| matches!(c.state.as_deref(), Some("running")))
                        .flat_map(|c| c.id.as_deref())
                        .collect::<Vec<_>>();

                    futures::future::try_join_all(
                        remove_containers
                            .iter()
                            .map(|c| client.stop_container(c, None)),
                    )
                    .await
                    .map_err(|error| TestcontainersError::Other(Box::new(error)))?;

                    #[cfg(feature = "tracing")]
                    if !remove_containers.is_empty() {
                        tracing::warn!(name = "stop running containers", result = ?remove_containers);
                    }
                }

                let _result = client
                    .prune_containers(Some(PruneContainersOptions { filters }))
                    .await
                    .map_err(|err| TestcontainersError::Other(Box::new(err)))?;

                #[cfg(feature = "tracing")]
                if _result
                    .containers_deleted
                    .as_ref()
                    .is_some_and(|c| !c.is_empty())
                {
                    tracing::warn!(name = "prune existed containers", result = ?_result);
                }
            }

            let result = self.with_labels([
                (testcontainers_prune_key, "true"),
                (testcontainers_project_key, scope),
                (testcontainers_container_key, container_label),
            ]);

            Ok(result)
        }
    }
}

impl<R, I> ImagePruneExistedLabelExt<I> for R
where
    R: Sized + ImageExt<I> + Send,
    I: Image,
{
}

pub trait ImageDefaultLogConsumerExt<I>: Sized + ImageExt<I>
where
    I: Image,
{
    /// Given a container, this method will return a container request with a default log consumer.
    ///
    /// Example:
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    /// use testcontainers::{core::{IntoContainerPort, WaitFor}, runners::AsyncRunner, GenericImage, ImageExt};
    /// use testcontainers_ext::ImageDefaultLogConsumerExt;
    /// use anyhow::Result;
    ///
    /// async fn test () -> Result<()> {
    ///     let container = GenericImage::new("redis", "7.2.4")
    ///         .with_exposed_port(6379.tcp())
    ///         .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
    ///         .with_default_log_consumer()
    ///         .start()
    ///         .await?;
    ///   Ok(())
    /// }
    ///
    /// Runtime::new().unwrap().block_on(test()).unwrap();
    /// ```
    ///
    fn with_default_log_consumer(self) -> ContainerRequest<I> {
        use testcontainers::core::logs::consumer::logging_consumer::LoggingConsumer;

        self.with_log_consumer(
            LoggingConsumer::new()
                .with_stdout_level(log::Level::Info)
                .with_stderr_level(log::Level::Error),
        )
    }
}

impl<R, I> ImageDefaultLogConsumerExt<I> for R
where
    R: Sized + ImageExt<I>,
    I: Image,
{
}
