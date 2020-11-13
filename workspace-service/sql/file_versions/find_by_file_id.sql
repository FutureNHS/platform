SELECT file_versions.id,
    file_versions.file_title AS title,
    file_versions.created_at,
    users.email_address

FROM file_versions JOIN users ON file_versions.created_by = users.id
WHERE file_versions.file = $1
ORDER BY file_versions.version_number desc
