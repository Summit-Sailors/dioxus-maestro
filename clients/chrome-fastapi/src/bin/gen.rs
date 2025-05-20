const OPENAPI_JSON_FILE: &str = "./openapi.json";

fn main() {
	println!("cargo:rerun-if-changed={}", OPENAPI_JSON_FILE);
	let file = std::fs::File::open(OPENAPI_JSON_FILE).expect("expected json file");
	let spec = serde_json::from_reader(file).expect("expected serde json to be read");
	let mut generator = progenitor::Generator::default();

	let tokens = generator.generate_tokens(&spec).expect("couldnt generate token stream");
	let ast = syn::parse2(tokens).expect("couldnt parse token stream to ast file");
	let content = prettyplease::unparse(&ast);

	let outdir = std::env::var("OUT_DIR").expect("expected OUT_DIR env var");
	// let mut out_file = std::path::Path::new(&outdir).to_path_buf();
	let mut out_file = std::path::Path::new("./src").to_path_buf();
	out_file.push("codegen.rs");

	std::fs::write(out_file, content).expect("couldnt write to file");

	println!("cargo:rustc-env=OUT_DIR={outdir}");
}
