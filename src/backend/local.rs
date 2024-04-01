use crate::library::Library;
use chrono::prelude::{DateTime, NaiveDate, Utc};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::{create_dir, metadata};
use std::io::prelude::*;
use std::path::PathBuf;

pub struct LocalBackend {
    url: PathBuf,
    metadata: Option<LocalMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocalMetadata {
    created_date: DateTime<Utc>,
    libraries: Vec<String>,
}

impl LocalMetadata {
    fn new(backend: &LocalBackend) -> Self {
        let created_date = Utc::now();
        let metadata = Self {
            created_date,
            libraries: Vec::new(),
        };

        let mut metadata_path = backend.url.clone();
        metadata_path.push("METADATA");
        let mut metadata_file = File::create(metadata_path).expect("");
        let metadata_serde = serde_json::to_string(&metadata).expect("");
        let metadata_bytes = metadata_serde.as_bytes();
        let _ = metadata_file.write_all(metadata_bytes);

        metadata
    }

    fn read(backend: &LocalBackend) -> Self {
        let mut metadata_path = backend.url.clone();
        metadata_path.push("METADATA");

        let mut metadata_file = File::open(metadata_path).expect("");
        let mut content = String::new();
        metadata_file.read_to_string(&mut content).expect("");
        serde_json::from_str(&content).expect("")
    }
}

impl LocalBackend {
    pub fn from(path: String) -> Self {
        let url = PathBuf::from(path);

        let url_path = url.as_path();
        if metadata(url_path).is_ok() {
            Self::connect(url)
        } else {
            Self::new(url)
        }
    }

    pub fn new(url: PathBuf) -> Self {
        // create local repository
        let url_path = url.as_path();
        let _ = create_dir(url_path);

        let mut backend = Self {
            url,
            metadata: None,
        };

        // read metadata file
        let metadata = LocalMetadata::new(&backend);
        backend.metadata = Some(metadata);

        backend
    }

    pub fn connect(url: PathBuf) -> Self {
        let mut backend = Self {
            url,
            metadata: None,
        };

        // read metadata file
        let metadata = LocalMetadata::read(&backend);
        backend.metadata = Some(metadata);

        backend
    }

    pub async fn create_library(&self, name: &String) {
        let mut library_url = self.url.clone();
        library_url.push(name);
        let library_url_path = library_url.as_path();
        let _ = create_dir(library_url_path);
    }

    pub async fn write(&self, library: &Library, df: DataFrame) {
        println!(
            "writing dataframe to {}/{}",
            self.url.display(),
            library.name
        );
    }

    pub async fn read(&self, library: &Library) -> DataFrame {
        println!("read from {}/{}", self.url.display(), library.name);
        let df: DataFrame = df!(
            "integer" => &[1, 2, 3, 4, 5],
            "date" => &[
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 4).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 5).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            ],
            "float" => &[4.0, 5.0, 6.0, 7.0, 8.0]
        )
        .expect("");
        df
    }

    pub async fn delete(&self, library: &Library) {
        println!("delete from {}/{}", self.url.display(), library.name);
    }
}
