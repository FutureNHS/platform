INSERT INTO workspace (title, long_description)
VALUES ($1, $2)
RETURNING id, title, long_description
