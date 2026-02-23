fn main() {
    let proto_files = &[
        "proto/envelope.proto",
        "proto/telemetry.proto",
        "proto/command.proto",
        "proto/config.proto",
        "proto/engineering.proto",
        "proto/common.proto",
    ];
    let proto_include_dirs = &["proto"];

    prost_build::Config::new()
        .compile_protos(proto_files, proto_include_dirs)
        .expect("failed to compile protos");

    println!("cargo:rerun-if-changed=proto/");
}
