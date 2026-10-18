pub mod model;
pub mod schema;

#[cfg(feature = "local_fs")]
pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("migrations");

#[cfg(all(test, feature = "local_fs"))]
#[path = "local_workspace_tests.rs"]
mod local_workspace_tests;
