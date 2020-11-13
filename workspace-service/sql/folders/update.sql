UPDATE folders
SET title = COALESCE($2, title),
    description = COALESCE($3, description),
    role_required = COALESCE($4, role_required)
WHERE id = $1
RETURNING *
