use std::i32;
use colored::*;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployer {
	pub repository: Option<String>,
	pub hostname: Option<String>,
	pub username: Option<String>,
	pub password: Option<String>,
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
	pub pre_deploy_commands: Option<Vec<String>>,
	pub post_deploy_commands: Option<Vec<String>>,
}

impl Deployer {
	pub fn new() -> Self {
		Self {
			repository: None,
			hostname: None,
			username: None,
			password: None,
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
			pre_deploy_commands: None,
			post_deploy_commands: None,
		}
	}
	pub fn configure(&mut self, path: &str) {
		if !std::path::Path::new(path).exists() {
			println!("[!] {}: {}", "File does not exist".red(), path);
			std::process::exit(1);
		}
		let contents = std::fs::read_to_string(path);
		match contents {
			Ok(contents) => {
				let deployer = serde_yaml::from_str(&contents);
				match deployer {
					Ok(deployer) => {
						println!("[+] {}: {}", "Config file loaded".green(), path);
						let deployer: Deployer = deployer;
						self.repository = deployer.repository;
						self.hostname = deployer.hostname;
						self.username = deployer.username;
						self.password = deployer.password;
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
						self.pre_deploy_commands = deployer.pre_deploy_commands;
						self.post_deploy_commands = deployer.post_deploy_commands;
					}
					Err(_) => {
						println!("[!] {}", "Failed to parse config file".red());
						std::process::exit(1);
					}
				}
			}
			Err(e) => {
				println!("[!] {}: {}", "Failed to read file".red(), e);
				std::process::exit(1);
			}
		}
	}
	pub fn deploy(&self) {
		println!("[~] {}", "Deploying...".bright_cyan());
		
		let hostname = self.hostname.clone();
		let username = self.username.clone();
		let password = self.password.clone();
		let tcp = TcpStream::connect(hostname.unwrap().as_str()).unwrap();
		let mut sess = Session::new().unwrap();
		sess.set_tcp_stream(tcp);
		sess.handshake().unwrap();
		sess.userauth_password(username.unwrap().as_str(), password.unwrap().as_str()).unwrap();
		
		match sess.authenticated() {
			true => {
				self.execute_list(sess.clone(), self.pre_deploy_commands.clone().unwrap());
				// todo: deploy commands
				self.execute_list(sess.clone(), self.post_deploy_commands.clone().unwrap());
			}
			false => {
				panic!("[!] {}", "Failed to authenticate".red());
			}
		}
	}
	pub fn execute_list(&self, sess: Session, commands: Vec<String>) {
		for command in commands {
			let _ = Self::execute_command(sess.clone(), command.clone());
		}
	}
	
	fn execute_command(sess: Session, command: String) -> i32 {
		let mut channel = sess.channel_session().unwrap();
		channel.exec(command.as_str()).unwrap();
		let mut buffer = String::new();
		channel.read_to_string(&mut buffer).unwrap();
		channel.wait_close().unwrap();
		let exit_status = channel.exit_status().unwrap();
		match exit_status {
			0 => println!("[+] {}: {}", "Command executed successfully".bright_green(), command.clone()),
			_ => println!("[!] {}: {}", "Command failed to execute".red(), command.clone()),
		}
		if buffer.len() > 0 {
			println!("\n{}", buffer.yellow());
		}
		return channel.exit_status().unwrap();
	}
}
