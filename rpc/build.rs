use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("microblog_descriptor.bin"))
        .compile_protos(
            &["proto/microblog/rpc/v1/microblog.proto"],
            &["protomicroblog/rpc/v1"],
        )
        .unwrap();
}
