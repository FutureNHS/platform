use super::azure;
use anyhow::Result;
use async_graphql::{Context, FieldResult, Object};
use url::Url;
use uuid::Uuid;

#[derive(Default)]
pub struct FileUploadUrlsQuery;

#[Object]
impl FileUploadUrlsQuery {
    #[field(desc = "Get URLs for uploading files")]
    async fn file_upload_urls(&self, context: &Context<'_>, count: u8) -> FieldResult<Vec<Url>> {
        let config = context.data()?;

        let urls = (0..count)
            .map(|_i| {
                let name = Uuid::new_v4();
                azure::create_upload_sas(config, &name)
            })
            .collect::<Result<Vec<Url>>>()?;

        Ok(urls)
    }
}
