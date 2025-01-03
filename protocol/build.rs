extern crate prost_build;

fn main() {
    let proto_files = &[
        "proto/chain.proto",
        "proto/client.proto",
        "proto/msg.proto",
        "proto/card.proto",
    ];
    let proto_include_dirs = &["proto/"];

    prost_build::Config::new()
        .out_dir("src/generated")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(proto_files, proto_include_dirs)
        .expect("Failed to compile Protobuf files");
}