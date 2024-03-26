use crate::backend::Backend;
use crate::library::Library;

pub struct Client {
    pub backend: Backend,
}


impl Client {
    pub fn from(url: String) -> Client {
        let backend = Backend::from(url);
        Client { backend }
    }

    pub async fn create_library(&self, name: String) -> Library {
        Library::create(&self.backend, name).await
    }
}





// pub struct Client {
//     backend: LocalBackend,
// }
// 
// impl Client {
//     pub fn from(url: String) -> Self {
//         let backend = LocalBackend::from(url);
//         Client { backend }
//     }
// 
//     pub fn create_library(&self, library_name: String) {
//         self.backend.create_library(library_name);
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     use crate::client::Client;
// 
//     #[test]
//     fn create_client() {
//         let url = "my_url";
//         let client = Client::from(url.to_string());
//         assert_eq!(
//             client.backend.path.to_str().unwrap(),
//             "/tmp/my_url".to_string()
//         );
//     }
// }
