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

#[cfg(feature = "server")]
async fn get_bicmid_from_session(session: &tower_sessions::Session) -> Result<String> {
    if let Some(bicmid) = session.get::<String>("bicmid").await? {
        Ok(bicmid)
    } else {
        Err(anyhow::anyhow!("Failed to get the bicmid from session"))
    }
}

/// Query the database and return the last 20 cats and their url
#[get("/api/v1/cats?off=offset" , session: tower_sessions::Session)]
pub async fn list_cats(offset: usize) -> Result<Vec<(i64, String)>> {
    let bicmid = get_bicmid_from_session(&session).await?;
    let offset: i64 = offset.try_into()?;
    let r = {
        let mut tx = DB.begin().await?;
        //
        let cats = sqlx::query(concat!(
            r#"SELECT Cat.id, UrlOrigin.value, Cat.url_path FROM Cat"#,
            r#" INNER JOIN Bicmid ON Cat.bicmid_id = Bicmid.id"#,
            r#" INNER JOIN UrlOrigin ON Cat.url_origin_id = UrlOrigin.id"#,
            r#" WHERE Bicmid.value = ?"#,
            r#" ORDER BY Cat.id DESC LIMIT 20 OFFSET ?"#
        ))
        .bind(bicmid)
        .bind(offset)
        .fetch_all(&mut *tx)
        .await?
        .iter()
        .map(|row| {
            (
                row.get::<i64, _>(0),
                format!("{}{}", row.get::<String, _>(1), row.get::<String, _>(2)),
            )
        })
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
#[post("/api/v1/count_of_cats" , session: tower_sessions::Session)]
pub async fn count_of_cats(_x: String) -> Result<usize> {
    let bicmid = get_bicmid_from_session(&session).await?;
    let r = {
        let mut tx = DB.begin().await?;
        //
        let r = sqlx::query(concat!(
            r#"SELECT count(*) FROM Cat"#,
            r#" INNER JOIN Bicmid ON Cat.bicmid_id = Bicmid.id"#,
            r#" WHERE Bicmid.value = ?"#
        ))
        .bind(bicmid)
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
#[delete("/api/v1/cats/{id}" , session: tower_sessions::Session)]
pub async fn delete_cat(id: i64) -> Result<()> {
    let bicmid = get_bicmid_from_session(&session).await?;
    {
        let mut tx = DB.begin().await?;
        //
        sqlx::query(concat!(
            r#"DELETE FROM Cat"#,
            r#" WHERE id IN ("#,
            r#" SELECT Cat.id FROM Cat"#,
            r#" INNER JOIN Bicmid ON Cat.bicmid_id = Bicmid.id"#,
            r#" WHERE Bicmid.value = ? AND Cat.id = ?"#,
            r#" )"#,
        ))
        .bind(bicmid)
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
#[post("/api/v1/cats" , session: tower_sessions::Session)]
pub async fn save_cat(image: String) -> Result<()> {
    let bicmid = get_bicmid_from_session(&session).await?;
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
    let (url_origin, url_path) = {
        let url = image.as_str();
        if let Some(off1) = url.find("//") {
            if let Some(off2) = url[(off1 + 2)..].find("/") {
                (&url[..(off1 + 2 + off2)], &url[(off1 + 2 + off2)..])
            } else {
                (url, "")
            }
        } else {
            (url, "")
        }
    };
    loop {
        let mut tx = DB.begin().await?;
        //
        let bicmid_id = get_or_store_bicmid(&mut tx, &bicmid).await?;
        if bicmid_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        let url_origin_id = get_or_store_url_origin(&mut tx, &url_origin).await?;
        if url_origin_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        sqlx::query(concat!(
            r#"INSERT INTO Cat"#,
            r#" (bicmid_id, url_origin_id, url_path)"#,
            r#" VALUES (?, ?, ?)"#
        ))
        .bind(&bicmid_id)
        .bind(&url_origin_id)
        .bind(&url_path)
        .execute(&mut *tx)
        .await?;
        //
        tx.commit().await?;
        break;
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

#[cfg(feature = "server")]
macro_rules! simple_get_or_store {
    ($func:ident, $tbl: expr) => {
        async fn $func(tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, val: &str) -> Result<i64> {
            let mut tbl_id = -1;
            let r = sqlx::query(concat!(r#"SELECT id FROM "#, $tbl, r#" WHERE value = ?"#))
                .bind(val)
                .fetch_one(&mut **tx)
                .await;
            if let Ok(row) = r {
                tbl_id = row.get(0);
            } else if let Err(sqlx::Error::RowNotFound) = r {
                let r = sqlx::query(concat!(r#"INSERT INTO "#, $tbl, r#" (value) VALUES (?)"#))
                    .bind(val)
                    .execute(&mut **tx)
                    .await?;
                tbl_id = r.last_insert_rowid();
            } else if let Err(e) = r {
                return Err(e.into());
            }
            Ok(tbl_id)
        }
    };
}

#[cfg(feature = "server")]
simple_get_or_store!(get_or_store_bicmid, "Bicmid");

#[cfg(feature = "server")]
simple_get_or_store!(get_or_store_url_origin, "UrlOrigin");

// Create tables if it doesn't already exist
#[cfg(feature = "server")]
async fn create_tables(pool: &sqlx::sqlite::SqlitePool) -> Result<()> {
    // table: `Cat`, `Bicmid`, `UrlOrigin`
    const SQL: &str = concat!(
        r#"CREATE TABLE IF NOT EXISTS Cat ("#,
        r#" id INTEGER PRIMARY KEY,"#,
        r#" bicmid_id INTEGER NOT NULL,"#,
        r#" create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,"#,
        r#" url_origin_id INTEGER NOT NULL,"#,
        r#" url_path  TEXT NOT NULL"#,
        r#");"#,
        "\n",
        r#"CREATE INDEX IF NOT EXISTS Cat_bicmid_id ON Cat (bicmid_id);"#,
        "\n",
        r#"CREATE TABLE IF NOT EXISTS Bicmid ("#,
        r#" id INTEGER PRIMARY KEY AUTOINCREMENT,"#,
        r#" create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,"#,
        r#" value TEXT NOT NULL"#,
        r#");"#,
        "\n",
        r#"CREATE UNIQUE INDEX IF NOT EXISTS Bicmid_value ON Bicmid (value);"#,
        "\n",
        r#"INSERT INTO Bicmid (id, value)"#,
        r#" SELECT * FROM (SELECT 0, '') AS Bicmid"#,
        r#" WHERE NOT EXISTS (SELECT * FROM Bicmid WHERE id = 0);"#,
        "\n",
        r#"CREATE TABLE IF NOT EXISTS UrlOrigin ("#,
        r#" id INTEGER PRIMARY KEY AUTOINCREMENT,"#,
        r#" create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,"#,
        r#" value TEXT NOT NULL"#,
        r#");"#,
        "\n",
        r#"CREATE UNIQUE INDEX IF NOT EXISTS UrlOrigin_value ON UrlOrigin (value);"#,
        "\n",
        r#"INSERT INTO UrlOrigin (id, value)"#,
        r#" SELECT * FROM (SELECT 0, '') AS UrlOrigin"#,
        r#" WHERE NOT EXISTS (SELECT * FROM UrlOrigin WHERE id = 0);"#,
        "\n",
    );
    sqlx::query(SQL).execute(pool).await?;
    Ok(())
}
