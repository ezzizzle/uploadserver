use std::env;
use std::fs;

#[derive(Clone, Debug, Copy)]
pub struct Config<'a> {
    pub file_directory: &'a str,
    pub base_url: &'a str,
    pub token: &'a str,
    pub port: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::path::Path;

    #[test]
    fn test_config() {
        let rand_path: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
        let path = format!("/tmp/{}", rand_path);
        env::set_var("FILE_DIRECTORY", &path);
        env::set_var("BASE_URL", "https://files.aselford.dev");
        env::set_var("UPLOAD_TOKEN", "tokengoeshere");
        env::set_var("SERVER_PORT", "3030");

        let config = Config::new();

        assert_eq!(config.file_directory, &path);
        assert_eq!(config.base_url, "https://files.aselford.dev");
        assert_eq!(config.token, "tokengoeshere");
        assert_eq!(config.port, 3030);

        assert!(Path::new(&path).exists());
    }

    #[test]
    #[should_panic]
    fn test_bad_port() {
        env::set_var("FILE_DIRECTORY", "/tmp");
        env::set_var("BASE_URL", "https://files.aselford.dev");
        env::set_var("UPLOAD_TOKEN", "tokengoeshere");
        env::set_var("SERVER_PORT", "abc");

        let _ = Config::new();
    }
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        let file_directory_env: String = env::var("FILE_DIRECTORY")
            .expect("FILE_DIRECTORY is not set")
            .parse()
            .expect("FILE_DIRECTORY is not a string");

        let base_url_env: String = env::var("BASE_URL")
            .expect("BASE_URL is not set")
            .parse()
            .expect("BASE_URL is not a string");

        let token_env: String = env::var("UPLOAD_TOKEN")
            .expect("UPLOAD_TOKEN is not set")
            .parse()
            .expect("UPLOAD_TOKEN is not a string");

        let port = env::var("SERVER_PORT")
            .expect("SERVER_PORT is not set")
            .parse()
            .expect("SERVER_PORT must be a number");

        let config = Config {
            file_directory: Box::leak(file_directory_env.into_boxed_str()),
            base_url: Box::leak(base_url_env.into_boxed_str()),
            token: Box::leak(token_env.into_boxed_str()),
            port,
        };
        println!("Setting file_directory to: {}", config.file_directory);
        println!("Setting file_direbase_urltory to: {}", config.base_url);
        println!("Setting port to: {}", config.port);

        // Create the output directory or panic
        fs::create_dir_all(config.file_directory).unwrap();

        config
    }
}
