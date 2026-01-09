#[cfg(feature = "server")]
use anyhow::Result;

#[cfg(feature = "server")]
use std::path::PathBuf;

#[cfg(feature = "server")]
pub async fn session_store() -> Result<tower_sessions_sqlx_store::SqliteStore> {
    use tower_sessions_sqlx_store::SqliteStore;

    let pool = create_sqlx_pool().await?;
    let session_store = SqliteStore::new(pool).with_table_name("sessions").unwrap();
    // create table if not exists
    session_store.migrate().await?;
    Ok(session_store)
}

#[cfg(feature = "server")]
async fn create_sqlx_pool() -> Result<sqlx::sqlite::SqlitePool> {
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::str::FromStr;
    //
    let db_path = get_db_path_();
    let sq_uri = format!("sqlite://{}", db_path.display());
    //let sq_uri = "sqlite://target/sessions.sqlite3";
    // Open the database from the persisted "session.sqlite3" file
    let opts = SqliteConnectOptions::from_str(&sq_uri)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;
    Ok(pool)
}

#[cfg(feature = "server")]
fn get_db_path_() -> PathBuf {
    let key1 = "CATTONGUE_DB_SESSION_PATH";
    if let Ok(s) = std::env::var(key1) {
        return PathBuf::from(s);
    }
    let mut data_dir = super::data_base_dir();
    let db_file = "sessions.sqlite3".to_string();
    data_dir.push(db_file);
    data_dir
}
