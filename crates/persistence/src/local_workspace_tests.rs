use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::sql_types::BigInt;
use diesel::sqlite::SqliteConnection;

#[derive(QueryableByName)]
struct Count {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

#[test]
fn local_workspace_migration_preserves_legacy_data_and_rollback_only_removes_new_schema() {
    let mut connection = SqliteConnection::establish(":memory:").unwrap();
    connection
        .batch_execute(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE windows (id INTEGER PRIMARY KEY NOT NULL);
             CREATE TABLE tabs (id INTEGER PRIMARY KEY NOT NULL, window_id INTEGER NOT NULL,
                 FOREIGN KEY(window_id) REFERENCES windows(id));
             CREATE TABLE projects (path TEXT PRIMARY KEY NOT NULL);
             CREATE TABLE terminal_panes (id INTEGER PRIMARY KEY NOT NULL, session_uuid TEXT NOT NULL);
             INSERT INTO windows (id) VALUES (17);
             INSERT INTO tabs (id, window_id) VALUES (41, 17);
             INSERT INTO projects (path) VALUES ('legacy-project');
             INSERT INTO terminal_panes (id, session_uuid) VALUES (82, 'legacy-session');",
        )
        .unwrap();

    connection
        .batch_execute(include_str!(
            "../migrations/2026-10-18-000000_create_local_workspace/up.sql"
        ))
        .unwrap();

    assert_eq!(count(&mut connection, "windows"), 1);
    assert_eq!(count(&mut connection, "tabs"), 1);
    assert_eq!(count(&mut connection, "projects"), 1);
    assert_eq!(count(&mut connection, "terminal_panes"), 1);
    assert_eq!(count(&mut connection, "local_workspace_state"), 1);

    connection
        .batch_execute(include_str!(
            "../migrations/2026-10-18-000000_create_local_workspace/down.sql"
        ))
        .unwrap();

    assert_eq!(count(&mut connection, "windows"), 1);
    assert_eq!(count(&mut connection, "tabs"), 1);
    assert_eq!(count(&mut connection, "projects"), 1);
    assert_eq!(count(&mut connection, "terminal_panes"), 1);
    assert_eq!(
        diesel::sql_query(
            "SELECT COUNT(*) AS count FROM sqlite_master \
             WHERE type = 'table' AND name LIKE 'local_workspace_%'",
        )
        .get_result::<Count>(&mut connection)
        .unwrap()
        .count,
        0
    );
}

fn count(connection: &mut SqliteConnection, table: &str) -> i64 {
    diesel::sql_query(format!("SELECT COUNT(*) AS count FROM {table}"))
        .get_result::<Count>(connection)
        .unwrap()
        .count
}
