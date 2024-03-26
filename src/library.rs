use crate::backend::Backend;

pub struct Library<'a> {
    backend: &'a Backend,
    name: String,
}

impl Library<'_> {
    pub async fn create(backend: &Backend, name: String) -> Library {
        Library { backend, name }
    }

    pub async fn write(&self, data: String) {
        self.backend.write(&self.name, data).await
    }

    pub async fn read(&self) {
        self.backend.read(&self.name).await
    }
}
