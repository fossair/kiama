use std::path::PathBuf;


pub struct LocalBackend {
    url: PathBuf,
}

impl LocalBackend {
    pub fn from(path: String) -> LocalBackend {
        let url = PathBuf::from(path);

        LocalBackend { url }
    }

    pub async fn write(&self, library: &String, data: String) {
        println!("writing {} to {}/{}", data, self.url.display(), library);
    }

    pub async fn read(&self, library: &String) {
        println!("read from {}/{}", self.url.display(), library);
    }

    pub async fn delete(&self, library: &String) {
        println!("delete from {}/{}", self.url.display(), library);
    }
}
