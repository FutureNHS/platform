DELETE FROM folders
WHERE id = $1
RETURNING id, title, description, role_required, workspace
