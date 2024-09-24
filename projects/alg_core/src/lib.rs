//shity shader functions for cpus

//#[should_panic(expected = "Cannot normalize a zero vector in GLSL")]


pub mod uri;

pub mod vfs {
    //light weight file system for browser for load scripts

    
    pub struct File {
        pub parent_folder: String,
        pub name: String,
        pub content: Vec<u8>,
    }
    pub struct term_vfs {
        pub files: Vec<File>,
        pub folders: Vec<String>,
        pub current_dir: String,
    }
    impl term_vfs {
        pub fn new() -> Self {
            term_vfs {
                files: vec![],
                folders: vec![],
                current_dir: "/".to_string(),
            }
        }
        pub fn change_dir(&mut self, dir: &str) {
            //check if dir exists else print error
            match self.folders.iter().find(|&x| x == dir) {
                Some(_) => {
                    self.current_dir = dir.to_string();
                }
                None => {
                    println!("Error: Directory does not exist");
                }
            }
        }
        pub fn create_dir(&mut self, dir: &str) {
            //check if dir exists else print error
            match self.folders.iter().find(|&x| x == dir) {
                Some(_) => {
                    println!("Error: Directory already exists");
                }
                None => {
                    self.folders.push(dir.to_string());
                }
            }
        }
        pub fn create_file(&mut self, name:String,data:u8){
            //check if file exists else print error
            match self.files.iter().find(|&x| x.name == name) {
                Some(_) => {
                    println!("Error: File already exists");
                }
                None => {
                    self.files.push(File {
                        parent_folder: self.current_dir.clone(),
                        name: name,
                        content: vec![data],
                    });
                }
            }
        }
    }

    
}



#[cfg(test)]
mod tests {
    use super::vfs::term_vfs;
    //vfs test for browser that use rust as a scripting language
    #[test]
    fn test_vfs_new() {
        let vfs = term_vfs::new();
        assert_eq!(vfs.files.len(), 0);
        assert_eq!(vfs.folders.len(), 0);
        assert_eq!(vfs.current_dir, "/");
    }

    #[test]
    fn test_vfs_change_dir() {
        let mut vfs = term_vfs::new();
        vfs.folders.push("/home".to_string());
        vfs.change_dir("/home");
        assert_eq!(vfs.current_dir, "/home");
    }
    #[test]
    fn test_vfs_create_dir() {
        let mut vfs = term_vfs::new();
        vfs.create_dir("/home");
        assert!(vfs.folders.contains(&"/home".to_string()));
    }

    #[test]
    fn test_vfs_create_file() {
        let mut vfs = term_vfs::new();
        vfs.create_file("file1".to_string(), 1);
        assert!(vfs.files.iter().any(|file| file.name == "file1"));
    }


    use std::net::Ipv6Addr;
    use std::str::FromStr;
    use crate::uri::{Host, URI,UriGetterSetter};
    #[test]
    fn test_ipv6_addr() {
        let addr = Ipv6Addr::from_str("2001:0db8:85a3:0000:0000:8a2e:0370:7334").unwrap();
        assert_eq!(addr.to_string(), "2001:db8:85a3::8a2e:370:7334");
    }

    #[test]
    fn test_uri_to_string() {
        let uri = URI {
            scheme: "http".to_string(),
            user_info: Some("user:password".to_string()),
            host: Host::RegisteredName("example.com".to_string()),
            port: Some(8080),
            path: "/path".to_string(),
            query: Some("query".to_string()),
            fragment: Some("fragment".to_string()),
        };

        let expected = "http://user:password@example.com:8080/path?query#fragment";
        assert_eq!(uri.to_string(), expected);
    }
}