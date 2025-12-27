use anyhow::Result;

#[allow(unused_imports)]
use std::path::PathBuf;

#[allow(unused_imports)]
use dioxus::prelude::*;

// The database is only available to server code
#[cfg(any(feature = "server", feature = "desktop"))]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let db_path = {
            let mut data_dir = data_dir();
            let db_file = "cattongue.db";
            data_dir.push(db_file);
            data_dir
        };
        // Open the database from the persisted "cattongue.db" file
        let conn = rusqlite::Connection::open(db_path).expect("Failed to open database");

        // Create the "cats" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cats (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}

#[cfg(any(feature = "server", feature = "desktop"))]
fn data_dir() -> PathBuf {
    #[allow(unused_assignments)]
    let mut data_dir = PathBuf::from(".");
    #[cfg(feature = "desktop")]
    {
        data_dir = data_dir_on_desktop();
    }
    #[cfg(feature = "server")]
    {
        data_dir = PathBuf::from("/var/local/data/cat_tongue");
    }
    return data_dir;
}

#[cfg(feature = "desktop")]
fn data_dir_on_desktop() -> PathBuf {
    let mut data_dir = match std::env::home_dir() {
        Some(home) => home,
        None => {
            eprintln!("could NOT get `home_dir()`");
            PathBuf::from(".")
        }
    };
    data_dir.push(".data");
    data_dir.push("cat_tongue");
    let _ = std::fs::create_dir(&data_dir);
    data_dir
}

// Query the database and return the last 20 cats and their url
#[cfg_attr(not(feature = "desktop"), server)]
//#[get("/api/cats")]
pub async fn list_cats(offset: usize) -> Result<Vec<(usize, String)>> {
    let cats = DB.with(|db| {
        db.prepare("SELECT id, url FROM cats ORDER BY id DESC LIMIT 20 OFFSET ?1")
            .unwrap()
            .query_map([&offset], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(cats)
}

#[cfg_attr(not(feature = "desktop"), server)]
pub async fn count_of_cats() -> Result<usize> {
    let count: usize = DB.with(|db| {
        db.prepare("SELECT count(*) FROM cats")
            .unwrap()
            .query_one([], |row| Ok(row.get(0)?))
            .unwrap()
    });
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(count)
}

#[cfg_attr(not(feature = "desktop"), server)]
//#[delete("/api/cats/{id}")]
pub async fn delete_cat(id: usize) -> Result<()> {
    DB.with(|f| f.execute("DELETE FROM cats WHERE id = (?1)", [id]))?;
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(())
}

#[cfg_attr(not(feature = "desktop"), server)]
//#[post("/api/cats")]
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
    DB.with(|f| f.execute("INSERT INTO cats (url) VALUES (?1)", &[&image]))?;
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(())
}

#[allow(dead_code)]
#[cfg(feature = "backend_delay")]
async fn sleep_x(millis: u64) -> Result<()> {
    async_std::task::sleep(std::time::Duration::from_millis(millis)).await;
    Ok(())
}
