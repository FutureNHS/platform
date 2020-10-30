use super::{azure, db, validation, RequestingUser};
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject, ID};
use chrono::{DateTime, Utc};
use fnhs_event_models::{
    Event, EventClient, EventPublisher as _, FileCreatedData, FileUpdatedData,
};
use lazy_static::lazy_static;
use mime_db::extensions2;
use regex::Regex;
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref ALLOWED_FILENAME_CHARS: Regex = Regex::new(r"^[\w\s\.-]+$").expect("bad regex");
    static ref ALLOWED_EXTENSIONS: Regex = Regex::new(
        r"(?x)\.(
            (bmp)|
            (csv)|
            (doc)|
            (docx)|
            (eps)|
            (gif)|
            (jpeg)|
            (jpg)|
            (mov)|
            (odp)|
            (ods)|
            (odt)|
            (pdf)|
            (png)|
            (ppt)|
            (pptx)|
            (svg)|
            (txt)|
            (webp)|
            (xls)|
            (xlsx)
        )$"
    )
    .expect("bad regex");
}

/// A file
#[derive(SimpleObject)]
pub struct File {
    /// The id of the file
    pub id: ID,
    /// The title of the file
    pub title: String,
    /// The description of the file
    pub description: String,
    /// The id of the parent folder
    pub folder: ID,
    /// The name of the file
    pub file_name: String,
    /// The type of the file
    pub file_type: String,
    /// ID of the latest version of the file
    pub latest_version: ID,
    /// The time the file was created
    pub created_at: DateTime<Utc>,
    /// The time the file was modified
    pub modified_at: DateTime<Utc>,
    /// The time the file was deleted
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(InputObject, Debug, Validate)]
#[validate(schema(
    function = "new_file_name_matches_type",
    message = "the file extension is not valid for the specified MIME type",
))]
pub struct NewFile {
    pub title: String,
    pub description: String,
    pub folder: ID,
    #[validate(
        length(
            min = 5,
            max = 255,
            message = "the file name must be between 5 and 255 characters long"
        ),
        regex(
            path = "ALLOWED_FILENAME_CHARS",
            message = "the file name contains characters that are not alphanumeric, space, period, hyphen or underscore"
        ),
        regex(
            path = "ALLOWED_EXTENSIONS",
            message = "the file name does not have an allowed extension"
        )
    )]
    pub file_name: String,
    pub file_type: String,
    pub temporary_blob_storage_path: String,
}

fn new_file_name_matches_type(new_file: &NewFile) -> Result<(), ValidationError> {
    file_name_matches_file_type(&new_file.file_name, &new_file.file_type)
}

#[derive(InputObject, Debug, Validate)]
#[validate(schema(
    function = "new_file_version_name_matches_type",
    message = "the file extension is not valid for the specified MIME type",
))]
pub struct NewFileVersion {
    pub file: ID,
    pub latest_version: ID,
    pub title: Option<String>,
    pub description: Option<String>,
    pub folder: Option<ID>,
    #[validate(
        length(
            min = 5,
            max = 255,
            message = "the file name must be between 5 and 255 characters long"
        ),
        regex(
            path = "ALLOWED_FILENAME_CHARS",
            message = "the file name contains characters that are not alphanumeric, space, period, hyphen or underscore"
        ),
        regex(
            path = "ALLOWED_EXTENSIONS",
            message = "the file name does not have an allowed extension"
        )
    )]
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub temporary_blob_storage_path: Option<String>,
}

fn new_file_version_name_matches_type(new_file: &NewFileVersion) -> Result<(), ValidationError> {
    match (&new_file.file_name, &new_file.file_type) {
        (Some(file_name), Some(file_type)) => file_name_matches_file_type(file_name, file_type),
        (None, None) => Ok(()),
        _ => Err(ValidationError::new("file name and type are both required")),
    }
}

fn file_name_matches_file_type(file_name: &str, file_type: &str) -> Result<(), ValidationError> {
    let extension = ALLOWED_EXTENSIONS
        .captures(file_name)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str());
    if let Some(ext) = extension {
        if extensions2(file_type).any(|possible| ext == possible) {
            Ok(())
        } else {
            Err(ValidationError::new("bad MIME type"))
        }
    } else {
        Err(ValidationError::new("bad extension"))
    }
}

impl From<db::FileWithVersion> for File {
    fn from(d: db::FileWithVersion) -> Self {
        Self {
            id: d.id.into(),
            title: d.title,
            description: d.description,
            folder: d.folder.into(),
            file_name: d.file_name,
            file_type: d.file_type,
            latest_version: d.version.into(),
            created_at: d.created_at,
            modified_at: d.modified_at,
            deleted_at: d.deleted_at,
        }
    }
}

#[derive(Default)]
pub struct FilesQuery;

#[Object]
impl FilesQuery {
    /// Get all Files in a Folder
    async fn files_by_folder(&self, context: &Context<'_>, folder: ID) -> FieldResult<Vec<File>> {
        let pool = context.data()?;
        let folder = Uuid::parse_str(&folder)?;
        let files = db::FileWithVersion::find_by_folder(folder, pool).await?;

        Ok(files.into_iter().map(Into::into).collect())
    }

    /// Get file by ID
    async fn file(&self, context: &Context<'_>, id: ID) -> FieldResult<File> {
        self.get_file(context, id).await
    }

    #[graphql(entity)]
    async fn get_file(&self, context: &Context<'_>, id: ID) -> FieldResult<File> {
        let pool = context.data()?;
        let id = Uuid::parse_str(&id)?;
        let file = db::FileWithVersion::find_by_id(id, pool).await?;
        Ok(file.into())
    }
}

#[derive(Default)]
pub struct FilesMutation;

#[Object]
impl FilesMutation {
    /// Create a new file (returns the created file)
    async fn create_file(&self, context: &Context<'_>, new_file: NewFile) -> FieldResult<File> {
        let pool = context.data()?;
        let azure_config = context.data()?;
        let requesting_user = context.data::<super::RequestingUser>()?;
        let event_client: &EventClient = context.data()?;

        create_file(new_file, pool, azure_config, requesting_user, event_client).await
    }

    /// Create a new file version (returns the updated file)
    ///
    /// This will update the specified properties only and will take unspecified properties from
    /// the latest version.
    ///
    /// Both file and latest version are required. The operation will fail if the specified latest
    /// version is no longer the latest version of the file.
    async fn create_file_version(
        &self,
        context: &Context<'_>,
        new_version: NewFileVersion,
    ) -> FieldResult<File> {
        let pool = context.data()?;
        let azure_config = context.data()?;
        let requesting_user = context.data::<super::RequestingUser>()?;
        let event_client: &EventClient = context.data()?;

        create_file_version(
            new_version,
            pool,
            azure_config,
            requesting_user,
            event_client,
        )
        .await
    }

    /// Deletes a file by id(returns delete file
    async fn delete_file(&self, context: &Context<'_>, id: ID) -> FieldResult<File> {
        let pool = context.data()?;
        let requesting_user = context.data::<super::RequestingUser>()?;
        let user = db::User::find_by_auth_id(&requesting_user.auth_id, pool).await?;
        let file: File = db::FileWithVersion::delete(Uuid::parse_str(&id)?, user.id, pool)
            .await?
            .into();

        Ok(file)
    }
}

async fn create_file(
    new_file: NewFile,
    pool: &PgPool,
    azure_config: &azure::Config,
    requesting_user: &RequestingUser,
    event_client: &EventClient,
) -> FieldResult<File> {
    new_file
        .validate()
        .map_err(validation::ValidationError::from)?;

    let folder_id = Uuid::parse_str(&new_file.folder)?;
    let user = db::User::find_by_auth_id(&requesting_user.auth_id, pool).await?;
    let destination = azure::copy_blob_from_url(
        &Url::parse(&new_file.temporary_blob_storage_path)?,
        azure_config,
    )
    .await?;

    let folder = db::Folder::find_by_id(folder_id, pool).await?;

    let file: File = db::FileWithVersion::create(
        db::CreateFileArgs {
            user_id: user.id,
            folder_id,
            title: &new_file.title,
            description: &new_file.description,
            file_name: &new_file.file_name,
            file_type: &new_file.file_type,
            blob_storage_path: &destination,
        },
        pool,
    )
    .await?
    .into();

    event_client
        .publish_events(&[Event::new(
            file.id.clone(),
            FileCreatedData {
                file_id: file.id.to_string(),
                created_at: file.created_at.to_rfc3339(),
                file_description: file.description.clone(),
                file_title: file.title.clone(),
                file_type: file.file_type.clone(),
                folder_id: folder_id.to_string(),
                user_id: user.id.to_string(),
                workspace_id: folder.workspace.to_string(),
                version_id: file.latest_version.to_string(),
            },
        )])
        .await?;

    Ok(file)
}

async fn create_file_version(
    new_version: NewFileVersion,
    pool: &PgPool,
    azure_config: &azure::Config,
    requesting_user: &RequestingUser,
    event_client: &EventClient,
) -> FieldResult<File> {
    new_version
        .validate()
        .map_err(validation::ValidationError::from)?;

    let current_file_id = Uuid::parse_str(&new_version.file)?;
    let current_latest_version_id = Uuid::parse_str(&new_version.latest_version)?;

    let current_file = db::FileWithVersion::find_by_id(current_file_id, pool).await?;
    if current_file.version != current_latest_version_id {
        // Early check to see if the latest version matches to avoid potentially copying the
        // file unnecessarily. There is still a chance someone else creates a new version in
        // the time from now until we store the new version in the DB. This will be caught by
        // `db::File::update_latest_version`.
        return Err("specified version is not the latest version of the file".into());
    }

    let user = db::User::find_by_auth_id(&requesting_user.auth_id, pool).await?;
    let folder_id = match &new_version.folder {
        Some(folder_id) => Uuid::parse_str(folder_id)?,
        None => current_file.folder,
    };

    let folder = db::Folder::find_by_id(folder_id, pool).await?;

    let destination = match &new_version.temporary_blob_storage_path {
        Some(temporary_blob_storage_path) => {
            azure::copy_blob_from_url(&Url::parse(temporary_blob_storage_path)?, azure_config)
                .await?
        }
        None => current_file.blob_storage_path,
    };

    let version_number = current_file.version_number + 1;
    let file: File = db::FileWithVersion::create_version(
        db::CreateFileVersionArgs {
            user_id: user.id,
            file_id: current_file_id,
            latest_version: current_latest_version_id,
            folder_id,
            title: new_version.title.as_ref().unwrap_or(&current_file.title),
            description: new_version
                .description
                .as_ref()
                .unwrap_or(&current_file.description),
            file_name: new_version
                .file_name
                .as_ref()
                .unwrap_or(&current_file.file_name),
            file_type: new_version
                .file_type
                .as_ref()
                .unwrap_or(&current_file.file_type),
            blob_storage_path: &destination,
            version_number,
        },
        pool,
    )
    .await?
    .into();

    event_client
        .publish_events(&[Event::new(
            file.id.clone(),
            FileUpdatedData {
                file_id: file.id.to_string(),
                file_description: file.description.clone(),
                file_title: file.title.clone(),
                file_type: file.file_type.clone(),
                folder_id: folder_id.to_string(),
                user_id: user.id.to_string(),
                workspace_id: folder.workspace.to_string(),
                version_id: new_version.latest_version.to_string(),
                version_number: version_number.into(),
                updated_at: file.modified_at.to_rfc3339(),
            },
        )])
        .await?;

    Ok(file)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graphql::test_mocks::*;
    use fnhs_event_models::EventData;
    use test_case::test_case;

    #[test_case("filename.doc", Some("application/msword") , None ; "good extension doc")]
    #[test_case("filename.docx", Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document") , None ; "good extension docx")]
    #[test_case("image.png", Some("image/png") , None ; "good mime type")]
    #[test_case("image.png", Some("image/gif") , Some("the file extension is not valid for the specified MIME type") ; "bad mime type")]
    #[test_case("filename.zip", None , Some("the file name does not have an allowed extension") ; "bad extension zip")]
    #[test_case("filename.txt", Some("text/plain") , None ; "good extension has dot")]
    #[test_case("filenametxt", None , Some("the file name does not have an allowed extension") ; "bad extension no dot")]
    #[test_case(".doc", None , Some("the file name must be between 5 and 255 characters long") ; "too short")]
    #[test_case("%.doc", None , Some("the file name contains characters that are not alphanumeric, space, period, hyphen or underscore") ; "bad char percent")]
    #[test_case("%", None , Some("the file name must be between 5 and 255 characters long, the file name contains characters that are not alphanumeric, space, period, hyphen or underscore, the file name does not have an allowed extension") ; "multiple errors")]
    #[test_case("🦀.doc", None , Some("the file name contains characters that are not alphanumeric, space, period, hyphen or underscore") ; "bad char emoji")]
    #[test_case("xx\u{0}.doc", None , Some("the file name contains characters that are not alphanumeric, space, period, hyphen or underscore") ; "null char")]
    fn validate_filename(
        file_name: &'static str,
        file_type: Option<&'static str>,
        expected: Option<&'static str>,
    ) {
        validate_newfile_filename(file_name, file_type, expected);
        validate_newfileversion_filename(file_name, file_type, expected);
    }

    fn validate_newfile_filename(
        file_name: &'static str,
        file_type: Option<&'static str>,
        expected: Option<&'static str>,
    ) {
        let new_file = NewFile {
            title: "".to_string(),
            description: "".to_string(),
            folder: "".into(),
            file_name: file_name.to_string(),
            file_type: file_type.unwrap_or("").to_string(),
            temporary_blob_storage_path: "".to_string(),
        };
        let actual = new_file
            .validate()
            .map_err(validation::ValidationError::from)
            .map_err(|e| format!("{}", e))
            .err();
        assert_eq!(actual.as_deref(), expected);
    }

    fn validate_newfileversion_filename(
        file_name: &'static str,
        file_type: Option<&'static str>,
        expected: Option<&'static str>,
    ) {
        let new_file_version = NewFileVersion {
            file: "".into(),
            latest_version: "".into(),
            title: None,
            description: None,
            folder: None,
            file_name: Some(file_name.to_string()),
            file_type: file_type.map(Into::into),
            temporary_blob_storage_path: None,
        };
        let actual = new_file_version
            .validate()
            .map_err(validation::ValidationError::from)
            .map_err(|e| format!("{}", e))
            .err();
        assert_eq!(actual.as_deref(), expected);
    }

    #[async_std::test]
    async fn create_file_works() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let azure_config = mock_azure_config()?;
        let requesting_user = mock_unprivileged_requesting_user();
        let (events, event_client) = mock_event_emitter();

        let result = create_file(
            NewFile {
                title: "title".into(),
                description: "description".into(),
                folder: "d890181d-6b17-428e-896b-f76add15b54a".into(),
                file_name: "file.txt".into(),
                file_type: "text/plain".into(),
                temporary_blob_storage_path: "http://localhost:10000/devstoreaccount1/upload/fake"
                    .into(),
            },
            &pool,
            &azure_config,
            &requesting_user,
            &event_client,
        )
        .await;

        assert_eq!(result.unwrap().title, "title");
        assert!(events
            .try_iter()
            .any(|e| matches!(e.data, EventData::FileCreated(_))));

        Ok(())
    }

    #[async_std::test]
    async fn create_file_version_works() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let azure_config = mock_azure_config()?;
        let requesting_user = mock_unprivileged_requesting_user();

        let file_id = Uuid::new_v4();
        let current_file = db::FileWithVersion::find_by_id(file_id, &pool).await?;
        let (events, event_client) = mock_event_emitter();

        let result = create_file_version(
            NewFileVersion {
                file: file_id.into(),
                latest_version: current_file.version.into(),
                title: Some("title".into()),
                description: None,
                folder: Some("d890181d-6b17-428e-896b-f76add15b54a".into()),
                file_name: Some("file.txt".into()),
                file_type: Some("text/plain".into()),
                temporary_blob_storage_path: Some(
                    "http://localhost:10000/devstoreaccount1/upload/new-fake".into(),
                ),
            },
            &pool,
            &azure_config,
            &requesting_user,
            &event_client,
        )
        .await
        .unwrap();

        assert_eq!(result.title, "title");
        assert_eq!(result.description, "fake file for tests");
        assert_eq!(result.folder, "d890181d-6b17-428e-896b-f76add15b54a");
        assert_eq!(result.file_name, "file.txt");
        assert_eq!(result.file_type, "text/plain");
        assert!(events
            .try_iter()
            .any(|e| matches!(e.data, EventData::FileUpdated(_))));

        Ok(())
    }

    #[async_std::test]
    async fn create_file_version_fails_if_latest_version_mismatch() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let azure_config = mock_azure_config()?;
        let requesting_user = mock_unprivileged_requesting_user();
        let (events, event_client) = mock_event_emitter();

        let file_id = Uuid::new_v4();
        let result = create_file_version(
            NewFileVersion {
                file: file_id.into(),
                latest_version: Uuid::new_v4().into(),
                title: Some("title".into()),
                description: None,
                folder: None,
                file_name: None,
                file_type: None,
                temporary_blob_storage_path: None,
            },
            &pool,
            &azure_config,
            &requesting_user,
            &event_client,
        )
        .await;

        assert_eq!(
            result.err().unwrap().message,
            "specified version is not the latest version of the file"
        );

        assert_eq!(0, events.try_iter().count());
        Ok(())
    }
}
