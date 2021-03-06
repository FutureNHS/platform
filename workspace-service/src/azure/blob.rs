use anyhow::{anyhow, Result};
use std::convert::{TryFrom, TryInto};
use url::Url;
#[cfg(not(test))]
use {
    anyhow::bail,
    async_compat::Compat,
    azure_sdk_core::{errors::AzureError, prelude::*},
    azure_sdk_storage_blob::{blob::CopyStatus, Blob},
};

#[derive(PartialEq, Debug)]
pub struct BlobUrlParts {
    pub account: String,
    pub container: String,
    pub blob: Option<String>,
}

impl TryFrom<&Url> for BlobUrlParts {
    type Error = anyhow::Error;

    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        let mut path_segments = value
            .path_segments()
            .ok_or_else(|| anyhow!("url has no path"))?;

        let account = if value.host_str() == Some("127.0.0.1") {
            path_segments
                .next()
                .ok_or_else(|| anyhow!("cannot get account from url"))?
                .to_string()
        } else {
            let host = value
                .host()
                .ok_or_else(|| anyhow!("cannot get host from url"))?
                .to_string();
            host.split('.')
                .next()
                .ok_or_else(|| anyhow!("cannot get storage account from url"))?
                .to_string()
        };

        let container = path_segments
            .next()
            .ok_or_else(|| anyhow!("cannot get container name from url"))?
            .to_string();
        let blob = path_segments.next().map(|s| s.to_string());

        Ok(Self {
            account,
            container,
            blob,
        })
    }
}

#[cfg(not(test))]
pub async fn copy_blob_from_url(url: &Url, azure_config: &super::Config) -> Result<String> {
    let input: BlobUrlParts = url.try_into()?;

    if input.account != azure_config.account {
        bail!("source file is from an unsupported storage account");
    }

    if input.container != azure_config.upload_container {
        bail!("source file is from an unsupported container");
    }

    let mut source_url = url.clone();
    source_url.set_query(None);
    let source_url = super::create_download_sas(azure_config, &source_url)?;

    let target_blob = input
        .blob
        .ok_or_else(|| anyhow!("cannot get blob name from url"))?;

    let result = Compat::new(
        azure_config
            .client()
            .copy_blob_from_url()
            .with_container_name(&azure_config.files_container)
            .with_blob_name(&target_blob)
            .with_source_url(source_url.as_str())
            .with_is_synchronous(true)
            .finalize(),
    )
    .await;
    let copy_status = match result {
        Ok(response) => Ok(response.copy_status),
        // Azurite doesn't return a `x-ms-copy-status` header for synchronous blob copy, but the
        // Azure SDK for Rust expects it. The operation does succeed, though. Since we don't care
        // about the response properties, we can look for the exact error and ignore it.
        // Upstream issue: https://github.com/Azure/Azurite/issues/603
        Err(AzureError::HeaderNotFound(header))
            if azure_config.is_local_emulator() && header == "x-ms-copy-status" =>
        {
            Ok(CopyStatus::Success)
        }
        Err(err) => Err(err),
    }?;

    match copy_status {
        CopyStatus::Success => Ok(format!(
            "{}/{}",
            azure_config.files_container_url, target_blob
        )),
        _ => bail!("Sync copy did not complete: {}", copy_status),
    }
}

// Fake implementation for tests. If you want integration tests that exercise the database,
// see https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html.
#[cfg(test)]
pub async fn copy_blob_from_url(_url: &Url, _azure_config: &super::Config) -> Result<String> {
    Ok("http://localhost:10000/devstoreaccount1/files/fake".into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_from_url() {
        let url =
            &Url::parse("https://fnhsfilesdevstu.blob.core.windows.net/upload/my_blob").unwrap();
        let actual: BlobUrlParts = url.try_into().unwrap();
        let expected = BlobUrlParts {
            account: "fnhsfilesdevstu".to_string(),
            container: "upload".to_string(),
            blob: Some("my_blob".to_string()),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn extract_from_url_without_blob() {
        let url = &Url::parse("https://fnhsfilesdevstu.blob.core.windows.net/upload").unwrap();
        let actual: BlobUrlParts = url.try_into().unwrap();
        let expected = BlobUrlParts {
            account: "fnhsfilesdevstu".to_string(),
            container: "upload".to_string(),
            blob: None,
        };
        assert_eq!(actual, expected);
    }
}
