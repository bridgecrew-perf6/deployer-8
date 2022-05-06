use deployer::Deployer;

fn main() {
	let config = match std::env::args().skip(1).next() {
		Some(path) => path,
		None => "deployer.yml".to_string()
	};

	let mut deployer = Deployer::new();
	deployer.configure(config.as_str());
	deployer.deploy();
}
