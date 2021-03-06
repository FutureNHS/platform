mod blob;
mod sas;

use anyhow::{bail, Result};
use azure_sdk_storage_core::{client, key_client::KeyClient, shared_access_signature::SasProtocol};
pub use blob::copy_blob_from_url;
use blob::BlobUrlParts;
pub use sas::{create_download_sas, create_upload_sas};
use std::convert::TryInto;
use url::Url;

/// Azurite (the local Azure Storage emulator) comes with a [default storage account], which has a
/// hardcoded key. This key is also used by the Azure SDK for Rust when we create a client using
/// `with_emulator`.
///
/// [default storage account]: https://github.com/Azure/Azurite#default-storage-account
const DEFAULT_EMULATOR_ACCESS_KEY: &str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";

#[derive(Debug, Clone)]
pub struct Config {
    pub access_key: String,
    pub account: String,
    pub upload_container: String,
    pub upload_container_url: Url,
    pub files_container: String,
    pub files_container_url: Url,
}

impl Config {
    pub fn new(
        access_key: String,
        upload_container_url: Url,
        files_container_url: Url,
    ) -> Result<Self> {
        let upload: BlobUrlParts = (&upload_container_url).try_into()?;
        let files: BlobUrlParts = (&files_container_url).try_into()?;
        if upload.account != files.account {
            bail!("upload and files containers have different account names");
        }

        Ok(Self {
            access_key,
            account: upload.account,
            upload_container: upload.container,
            upload_container_url,
            files_container: files.container,
            files_container_url,
        })
    }

    pub fn is_local_emulator(&self) -> bool {
        self.access_key == DEFAULT_EMULATOR_ACCESS_KEY
    }

    pub fn sas_protocol(&self) -> SasProtocol {
        if self.is_local_emulator() {
            SasProtocol::HttpHttps
        } else {
            SasProtocol::Https
        }
    }

    pub fn client(&self) -> KeyClient {
        if self.is_local_emulator() {
            client::with_emulator(
                &Url::parse("http://127.0.0.1:10000").unwrap(),
                &Url::parse("http://127.0.0.1:10001").unwrap(),
            )
        } else {
            client::with_access_key(&self.account, &self.access_key)
        }
    }
}
