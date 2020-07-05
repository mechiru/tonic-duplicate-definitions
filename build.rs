fn main() {
    tonic_build::configure()
        .out_dir("src")
        .compile(&["proto/connect.proto"], &["proto/"])
        .unwrap();
}
