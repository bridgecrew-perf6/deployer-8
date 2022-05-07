use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployer {
	pub repository: Option<String>,
	pub hostname: Option<String>,
	pub remote_user: Option<String>,
	pub deploy_path: Option<String>,
	pub keep_releases: Option<String>,
	pub http_user: Option<String>,
	pub php_path: Option<String>,
	pub shared_files: Option<Vec<String>>,
	pub shared_dirs: Option<Vec<String>>,
	pub writable_use_sudo: Option<bool>,
	pub writable_recursive: Option<bool>,
	pub writable_chmod_mode: Option<String>,
	pub writable_dirs: Option<Vec<String>>,
	pub tasks: Option<Vec<String>>,
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
		self.repository = deployer.repository;
		self.hostname = deployer.hostname;
		self.remote_user = deployer.remote_user;
		self.deploy_path = deployer.deploy_path;
		self.keep_releases = deployer.keep_releases;
		self.http_user = deployer.http_user;
		self.php_path = deployer.php_path;
		self.shared_files = deployer.shared_files;
		self.shared_dirs = deployer.shared_dirs;
		self.writable_use_sudo = deployer.writable_use_sudo;
		self.writable_recursive = deployer.writable_recursive;
		self.writable_chmod_mode = deployer.writable_chmod_mode;
		self.writable_dirs = deployer.writable_dirs;
		self.tasks = deployer.tasks;
	}
	pub fn deploy(&self) {
		println!("Deploying...");
		println!("{:#?}", self);
	}
}
