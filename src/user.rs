extern crate prost_build;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("proto");
    std::fs::create_dir_all(&dest_path).unwrap();

    prost_build::Config::new()
        .out_dir(dest_path)
        .compile_protos(&["src/proto/user.proto"], &["src/proto"])
        .unwrap();
}
