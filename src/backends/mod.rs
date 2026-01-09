mod db_main;
pub use db_main::*;

mod db_session;
#[cfg(feature = "server")]
pub use db_session::*;

#[cfg(feature = "server")]
use std::path::PathBuf;

#[cfg(feature = "server")]
fn data_base_dir() -> PathBuf {
    let key = "CATTONGUE_DB_BASE_PATH";
    let data_dir = if let Ok(s) = std::env::var(key) {
        let pb = PathBuf::from(s);
        let _ = std::fs::create_dir_all(&pb);
        pb
    } else {
        data_dir()
    };
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
