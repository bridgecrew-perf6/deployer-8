use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::i32;
use std::io::prelude::*;
use std::io::stdout;
use std::net::TcpStream;
use colored::Colorize;

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
    pub fn deploy(&self) -> Result<()>{
        println!("[~] {}", "Deploying...".bright_cyan());

        let hostname = self.hostname.clone();
        let username = self.username.clone();
        let password = self.password.clone();
        let tcp = TcpStream::connect(hostname.unwrap().as_str()).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password(username.unwrap().as_str(), password.unwrap().as_str())
            .unwrap();

        match sess.authenticated() {
            true => {
                self.execute_list(
                    sess.clone(),
                    self.pre_deploy_commands.clone().unwrap(),
                    true,
                    true,
                );
                let wd = (self.deploy_path.clone()).unwrap();
                let repo = (self.repository.clone()).unwrap();
                let http_user = (self.http_user.clone()).unwrap();
                //todo: check if composer and npm/node is installed
                self.execute_list(
                    sess.clone(),
                    vec![
                        // format!("mkdir -p {}/shared", wd.clone()),
                        format!("rm -rf {}/release", wd),
                        format!("mkdir -p {}/release", wd),
                        format!("git clone {} {}/release", repo, wd),
                        format!("rm -f {}/release/composer.lock", wd),
                        format!("cd {}/release && composer install --no-interaction", wd),
                        format!("chown -R {} {}/release", http_user, wd),
                    ],
                    true,
                    false,
                );
                // format!("ln -s ...", wd.clone()),
                // format!("rm -rf {}/release", wd.clone()),
                self.execute_list(
                    sess.clone(),
                    self.post_deploy_commands.clone().unwrap(),
                    true,
                    true,
                );
            }
            false => {
                println!("[!] {}", "Failed to authenticate".red());
                std::process::exit(1);
            }
        }
	    Ok(())
    }
    pub fn execute_list(
        &self,
        sess: Session,
        commands: Vec<String>,
        show_input: bool,
        show_output: bool,
    ) {
        for command in commands {
            let _ = Self::execute_command(sess.clone(), command.clone(), show_input, show_output);
        }
    }

    fn execute_command(
        sess: Session,
        command: String,
        show_input: bool,
        show_output: bool,
    ) -> (i32, String) {
        if show_input == true {
            print!("[~] {}: {}", command.clone(), "Running".yellow());
            let _ = stdout().flush();
        }
        let mut channel = sess.channel_session().unwrap();
        channel.exec(command.as_str()).unwrap();
        let mut buffer = String::new();
        channel.read_to_string(&mut buffer).unwrap();
        channel.wait_close().unwrap();
        let exit_status = channel.exit_status().unwrap();
        if show_input == true {
            match exit_status {
                0 => println!("\r[+] {}: {}   ", command.clone(), "Success".bright_green()),
                _ => println!("\r[!] {}: {}   ", command.clone(), "Failed".red()),
            }
            let _ = stdout().flush();
        }
        if buffer.len() > 0 && show_output == true {
            println!("\n{}", buffer.yellow());
        }
        if exit_status != 0 {
            std::process::exit(1);
        }
        return (channel.exit_status().unwrap(), buffer);
    }
}
