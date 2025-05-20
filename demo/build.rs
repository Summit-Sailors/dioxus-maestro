#[dotenvy::load(path = "../.env")]
fn main() {
	let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
	if profile != "release" {
		println!("cargo:rustc-env=RUST_BACKTRACE=1");
		println!("cargo:rustc-env=CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true");
		println!("cargo:rerun-if-changed=../.env");
		println!("cargo:rerun-if-changed=./input.css");
	}

	#[cfg(feature = "server")]
	{
		for key in ["DATABASE_URL", "ANTHROPIC_API_KEY", "SERPAPI_API_KEY", "APALIS_DATABASE_URL"] {
			if let Ok(value) = std::env::var(key) {
				println!("cargo:rustc-env={}={}", key, value);
			} else {
				eprintln!("Warning: {} is not set!", key);
			}
		}
	}

	for key in ["SERVER_URL", "ENV"] {
		if let Ok(value) = std::env::var(key) {
			println!("cargo:rustc-env={}={}", key, value);
		} else {
			eprintln!("Warning: {} is not set!", key);
		}
	}
}
