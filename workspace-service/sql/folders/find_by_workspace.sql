SELECT  id, title, description, role_required, workspace
FROM folders
WHERE workspace = $1
