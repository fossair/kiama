use crate::backend::local::LocalBackend;

pub struct Library {
    backend: LocalBackend,
    name: String,
}

impl Library {
    pub async fn create(backend: LocalBackend, name: String) -> Library {
        Library { backend, name }
    }

    pub async fn write(&self, data: String) {
        self.backend.write(&self.name, data).await
    }

    pub async fn read(&self) {
        self.backend.read(&self.name).await
    }
}
