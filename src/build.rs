extern crate prost_build;

fn main() {
    prost_build::Config::new()
        .out_dir("src") // Output directory for the generated files
        .compile_protos(&["src/proto/user.proto"], &["src/proto"])
        .unwrap();
}
