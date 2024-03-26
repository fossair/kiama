pub struct Backend {
    url: String,
}

impl Backend {
    pub fn from(url: String) -> Backend {
        Backend { url }
    }

    pub async fn write(&self, library: &String, data: String) {
        println!("writing {} to {}/{}", data, self.url, library);
    }

    pub async fn read(&self, library: &String) {
        println!("read from {}/{}", self.url, library);
    }

    pub async fn delete(&self, library: &String) {
        println!("delete from {}/{}", self.url, library);
    }
}
