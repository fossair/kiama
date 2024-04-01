use crate::backend::local::LocalBackend;
use polars::prelude::DataFrame;

pub struct Library {
    backend: LocalBackend,
    pub name: String,
}

impl Library {
    pub async fn create(backend: LocalBackend, name: String) -> Library {
        let _ = backend.create_library(&name).await;
        Library { backend, name }
    }

    pub async fn write(&self, df: DataFrame) {
        self.backend.write(&self, df).await
    }

    pub async fn read(&self) -> DataFrame {
        self.backend.read(&self).await
    }
}
