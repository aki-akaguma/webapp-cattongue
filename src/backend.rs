use anyhow::Result;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use std::cell::RefCell;

#[cfg(feature = "server")]
use std::path::PathBuf;

// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: RefCell<rusqlite::Connection> = {
        let db_path = get_db_path_();
        // Open the database from the persisted "cattongue.db" file
        let conn = rusqlite::Connection::open(db_path).expect("Failed to open database");
        // Create tables if it doesn't already exist
        create_tables(&conn).unwrap();
        // Return the connection
        RefCell::new(conn)
    };
}

#[cfg(feature = "server")]
fn get_db_path_() -> PathBuf {
    let key1 = "CATTONGUE_DB_PATH";
    if let Ok(s) = std::env::var(key1) {
        return PathBuf::from(s);
    }
    let key2 = "CATTONGUE_DB_BASE_PATH";
    let mut data_dir = if let Ok(s) = std::env::var(key2) {
        let pb = PathBuf::from(s);
        let _ = std::fs::create_dir_all(&pb);
        pb
    } else {
        data_dir()
    };
    let key3 = "CATTONGUE_DB_FILE";
    let db_file = if let Ok(s) = std::env::var(key3) {
        s
    } else {
        "cattongue.db".to_string()
    };
    data_dir.push(db_file);
    data_dir
}

#[cfg(feature = "server")]
fn data_dir() -> PathBuf {
    let data_dir: PathBuf;
    #[cfg(not(feature = "backend_homedir"))]
    {
        data_dir = PathBuf::from("/var/local/data/cattongue");
        let _ = std::fs::create_dir_all(&data_dir);
    }
    #[cfg(feature = "backend_homedir")]
    {
        data_dir = data_dir_on_desktop();
    }
    return data_dir;
}

#[cfg(feature = "backend_homedir")]
#[cfg(feature = "server")]
fn data_dir_on_desktop() -> PathBuf {
    let mut data_dir = match std::env::home_dir() {
        Some(home) => home,
        None => {
            eprintln!("could NOT get `home_dir()`");
            PathBuf::from(".")
        }
    };
    data_dir.push(".data");
    data_dir.push("cattongue");
    let _ = std::fs::create_dir(&data_dir);
    data_dir
}

/// Query the database and return the last 20 cats and their url
#[get("/api/v1/cats?off=offset")]
pub async fn list_cats(offset: usize) -> Result<Vec<(usize, String)>> {
    let r = DB.with_borrow_mut(|db| {
        let tx = db.transaction()?;
        let cats = tx
            .prepare("SELECT id, url FROM Cat ORDER BY id DESC LIMIT 20 OFFSET ?1")
            .unwrap()
            .query_map([&offset], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        tx.rollback()?;
        Ok(cats)
    });
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    r
}

/// Query the database and return the count of cats
#[post("/api/v1/count_of_cats")]
pub async fn count_of_cats(_x: String) -> Result<usize> {
    let r = DB.with_borrow_mut(|db| {
        let tx = db.transaction()?;
        let r = tx
            .prepare("SELECT count(*) FROM Cat")
            .unwrap()
            .query_one([], |row| Ok(row.get(0)?))?;
        tx.rollback()?;
        Ok(r)
    });
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    r
}

/// Query the database and delete the cat
#[delete("/api/v1/cats/{id}")]
pub async fn delete_cat(id: usize) -> Result<()> {
    let r = DB.with_borrow_mut(|db| {
        let tx = db.transaction()?;
        tx.prepare("DELETE FROM Cat WHERE id = (?1)")
            .unwrap()
            .execute([id])?;
        tx.commit()?;
        Ok(())
    });
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    r
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
    let r = DB.with_borrow_mut(|db| {
        let tx = db.transaction()?;
        tx.prepare("INSERT INTO Cat (url) VALUES (?1)")
            .unwrap()
            .execute(&[&image])?;
        tx.commit()?;
        Ok(())
    });
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    r
}

#[cfg(feature = "backend_delay")]
async fn sleep_x(millis: u64) -> Result<()> {
    async_std::task::sleep(std::time::Duration::from_millis(millis)).await;
    Ok(())
}

// Create tables if it doesn't already exist
#[cfg(feature = "server")]
fn create_tables(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    // table: `Cat`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Cat (
             id INTEGER PRIMARY KEY,
             url TEXT NOT NULL
         );",
    )
    .unwrap();
    Ok(())
}
