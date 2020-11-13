INSERT INTO folders (title, description, role_required, workspace)
VALUES ($1, $2, $3, $4)
RETURNING id, title, description, role_required, workspace
