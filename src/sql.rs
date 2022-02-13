pub fn getAllTablesSql() -> &'static str {
    r#"
    SELECT
	COALESCE(json_agg(row_to_json(info)), '[]'::JSON)
FROM (
	SELECT
		table_name,
		table_schema
	FROM
		information_schema.tables
	WHERE
		table_type = 'BASE TABLE'
		AND table_schema NOT IN('pg_catalog', 'information_schema')) AS info;
    "#
}
