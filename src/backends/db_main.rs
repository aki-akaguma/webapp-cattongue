use anyhow::Result;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use std::path::PathBuf;

#[cfg(feature = "server")]
use dioxus::fullstack::Lazy;

#[cfg(feature = "server")]
use sqlx::Row;

// The database is only available to server code
#[cfg(feature = "server")]
static DB: Lazy<sqlx::SqlitePool> = Lazy::new(|| async move {
    let pool = create_sqlx_pool().await?;
    dioxus::Ok(pool)
});

#[cfg(feature = "server")]
async fn create_sqlx_pool() -> Result<sqlx::sqlite::SqlitePool> {
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::str::FromStr;
    //
    let db_path = get_db_path_();
    let sq_uri = format!("sqlite://{}", db_path.display());
    // Open the database from the persisted "cattongue.sqlite3" file
    let opts = SqliteConnectOptions::from_str(&sq_uri)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;
    // Create tables if it doesn't already exist
    create_tables(&pool).await?;
    Ok(pool)
}

#[cfg(feature = "server")]
fn get_db_path_() -> PathBuf {
    let key1 = "CATTONGUE_DB_PATH";
    if let Ok(s) = std::env::var(key1) {
        return PathBuf::from(s);
    }
    let mut data_dir = super::data_base_dir();
    let key3 = "CATTONGUE_DB_FILE";
    let db_file = if let Ok(s) = std::env::var(key3) {
        s
    } else {
        "cattongue.sqlite3".to_string()
    };
    data_dir.push(db_file);
    data_dir
}

#[post("/api/v1/session", session: tower_sessions::Session)]
pub async fn check_session(bicmid: String) -> Result<bool> {
    if let Some(session_bicmid) = session.get::<String>("bicmid").await? {
        dioxus_logger::tracing::debug!("bicmid: '{}' '{}'", &session_bicmid, &bicmid);
        Ok(bicmid.as_str() == session_bicmid.as_str())
    } else {
        dioxus_logger::tracing::debug!("insert bicmid: '{}'", &bicmid);
        session.insert("bicmid", &bicmid).await?;
        Ok(true)
    }
}

/// Query the database and return the last 20 cats and their url
#[get("/api/v1/cats?off=offset")]
pub async fn list_cats(offset: usize) -> Result<Vec<(i64, String)>> {
    let offset: i64 = offset.try_into()?;
    let r = {
        let mut tx = DB.begin().await?;
        //
        let cats = sqlx::query(concat!(
            r#"SELECT id, url FROM Cat"#,
            r#" ORDER BY id DESC LIMIT 20 OFFSET ?"#
        ))
        .bind(offset)
        .fetch_all(&mut *tx)
        .await?
        .iter()
        .map(|row| (row.get::<i64, _>(0), row.get(1)))
        .collect();
        //
        tx.commit().await?;
        cats
    };
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(r)
}

/// Query the database and return the count of cats
#[post("/api/v1/count_of_cats")]
pub async fn count_of_cats(_x: String) -> Result<usize> {
    let r = {
        let mut tx = DB.begin().await?;
        //
        let r = sqlx::query(concat!(r#"SELECT count(*) FROM Cat"#,))
            .fetch_one(&mut *tx)
            .await?
            .get::<i64, _>(0) as usize;
        //
        tx.commit().await?;
        r
    };
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(r)
}

/// Query the database and delete the cat
#[delete("/api/v1/cats/{id}")]
pub async fn delete_cat(id: i64) -> Result<()> {
    {
        let mut tx = DB.begin().await?;
        //
        sqlx::query(concat!(r#"DELETE FROM Cat WHERE id = ?"#))
            .bind(id)
            .execute(&mut *tx)
            .await?;
        //
        tx.commit().await?;
    }
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(())
}

/// Query the database and save the cat
#[post("/api/v1/cats")]
pub async fn save_cat(image: String) -> Result<()> {
    #[cfg(feature = "backend_text")]
    {
        use std::io::Write;
        //
        // Open the `cats.txt` file in append-only mode, creating it if it doesn't exist;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("cattongue.txt")
            .unwrap();
        // And then write a newline to it with the image url
        let _ = file.write_fmt(format_args!("{image}\n"));
    }
    //
    {
        let mut tx = DB.begin().await?;
        //
        sqlx::query(concat!(r#"INSERT INTO Cat (url) VALUES (?)"#))
            .bind(&image)
            .execute(&mut *tx)
            .await?;
        //
        tx.commit().await?;
    }
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(())
}

#[cfg(feature = "backend_delay")]
async fn sleep_x(millis: u64) -> Result<()> {
    async_std::task::sleep(std::time::Duration::from_millis(millis)).await;
    Ok(())
}

// Create tables if it doesn't already exist
#[cfg(feature = "server")]
async fn create_tables(pool: &sqlx::sqlite::SqlitePool) -> Result<()> {
    // table: `Cat`
    const SQL: &str = concat!(
        r#"CREATE TABLE IF NOT EXISTS Cat ("#,
        r#" id INTEGER PRIMARY KEY,"#,
        r#" url TEXT NOT NULL"#,
        r#");"#,
    );
    sqlx::query(SQL).execute(pool).await?;
    Ok(())
}
