use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployer {
	repository: Option<String>,
	hostname: Option<String>,
	remote_user: Option<String>,
	deploy_path: Option<String>,
	keep_releases: Option<String>,
	http_user: Option<String>,
	php_path: Option<String>,
	shared_files: Option<Vec<String>>,
	shared_dirs: Option<Vec<String>>,
	writable_use_sudo: Option<bool>,
	writable_recursive: Option<bool>,
	writable_chmod_mode: Option<String>,
	writable_dirs: Option<Vec<String>>,
	tasks: Option<Vec<String>>,
}

impl Deployer {
	pub fn new() -> Self {
		Self {
			repository: None,
			hostname: None,
			remote_user: None,
			deploy_path: None,
			keep_releases: None,
			http_user: None,
			php_path: None,
			shared_files: None,
			shared_dirs: None,
			writable_use_sudo: None,
			writable_recursive: None,
			writable_chmod_mode: None,
			writable_dirs: None,
			tasks: None,
		}
	}
	pub fn configure(&mut self, path: &str) {
		if !std::path::Path::new(path).exists() {
			panic!("File does not exist: {}", path);
		}
		let contents = std::fs::read_to_string(path).expect("Failed to read config file");
		let deployer: Deployer = serde_yaml::from_str(&contents).expect("Failed to parse config file");
		println!("{:#?}", deployer);
	}
	pub fn deploy(&self) {}
}

impl Default for Deployer {
	fn default() -> Self {
		Self {
			repository: Some("git@github.com:samirdjelal/deployer.git".to_string()),
			hostname: Some("localhost".to_string()),
			remote_user: Some("root".to_string()),
			deploy_path: Some("/data/wwwroot/default".to_string()),
			keep_releases: Some("5".to_string()),
			http_user: Some("daemon".to_string()),
			php_path: Some("/usr/local/php/bin/php".to_string()),
			shared_files: Some(vec![".env".to_string()]),
			shared_dirs: Some(vec!["storage".to_string()]),
			writable_use_sudo: Some(false),
			writable_recursive: Some(true),
			writable_chmod_mode: Some("0777".to_string()),
			writable_dirs: Some(vec![
				"bootstrap/cache".to_string(),
				"storage".to_string(),
				"storage/app".to_string(),
				"storage/app/public".to_string(),
				"storage/framework".to_string(),
				"storage/framework/cache".to_string(),
				"storage/framework/sessions".to_string(),
				"storage/framework/views".to_string(),
				"storage/logs".to_string(),
			]),
			tasks: Some(vec![
				"php artisan optimize".to_string(),
				"php artisan migrate".to_string(),
				"php artisan db:seed".to_string(),
			]),
		}
	}
}
