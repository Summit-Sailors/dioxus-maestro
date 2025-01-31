use std::process::Command;

#[dotenvy::load(path = "../.env")]
fn main() {
	let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
	if profile != "release" {
		println!("cargo:rustc-env=RUST_BACKTRACE=1");
		println!("cargo:rustc-env=CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true");
		println!("cargo:rerun-if-changed=../.env");
		println!("cargo:rerun-if-changed=./input.css");
		println!("cargo:rerun-if-changed=./tailwind.config.js");
	}

	#[cfg(feature = "server")]
	{
		for key in ["DATABASE_URL", "ANTHROPIC_API_KEY", "SERPAPI_API_KEY", "APALIS_DATABASE_URL"] {
			println!("cargo:rustc-env={}={}", key, std::env::var(key).unwrap());
		}
	}
	for key in ["SERVER_URL", "ENV"] {
		println!("cargo:rustc-env={}={}", key, std::env::var(key).unwrap());
	}
	let mut tailwind_cmd = Command::new("npx");
	tailwind_cmd.args(vec!["tailwindcss", "-i", "./input.css", "-o", "./assets/main.css"]);
	if profile == "release" {
		tailwind_cmd.arg("--minify");
	}
	let output = tailwind_cmd.output().expect("Failed to execute tailwindcss command");
	if !output.status.success() {
		panic!("Tailwind CSS compilation failed: {}", String::from_utf8_lossy(&output.stderr));
	}
}
