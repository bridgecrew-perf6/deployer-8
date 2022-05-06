extern crate yaml_rust;

use yaml_rust::{YamlLoader, Yaml};

pub struct Deployer {
	config: Vec<Yaml>,
	repository: Option<String>,
	hostname: Option<String>,
	remote_user: Option<String>,
	deploy_path: Option<String>,
	keep_releases: Option<i32>,
	http_user: Option<String>,
	php_path: Option<String>,
	shared_files: Option<Vec<String>>,
	shared_dirs: Option<Vec<String>>,
	writable_use_sudo: Option<bool>,
	writable_recursive: Option<bool>,
	writable_chmod_mode: Option<i32>,
	writable_dirs: Option<Vec<String>>,
	tasks: Option<Vec<String>>,
}

impl Deployer {
	pub fn new() -> Self {
		Self {
			config: vec![],
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
		let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
		self.config = docs.clone();
	}
	pub fn deploy(&self) {
		for doc in self.config.iter() {
			let yaml = doc.clone();
			match yaml.as_hash() {
				Some(hash) => {
					for (key, value) in hash.iter() {
						println!("{:?}: {:#?}", key, value);
					}
				}
				None => {
					println!("{:?}", yaml);
				}
			}
		}
	}
}

impl Default for Deployer {
	fn default() -> Self {
		Self {
			config: vec![],
			repository: Some("git@github.com:samirdjelal/deployer.git".to_string()),
			hostname: Some("localhost".to_string()),
			remote_user: Some("root".to_string()),
			deploy_path: Some("/data/wwwroot/default".to_string()),
			keep_releases: Some(5),
			http_user: Some("daemon".to_string()),
			php_path: Some("/usr/local/php/bin/php".to_string()),
			shared_files: Some(vec![".env".to_string()]),
			shared_dirs: Some(vec!["storage".to_string()]),
			writable_use_sudo: Some(false),
			writable_recursive: Some(true),
			writable_chmod_mode: Some(777),
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
