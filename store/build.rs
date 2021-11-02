use flatc_rust;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/store.proto")?;

    flatc_rust::run(flatc_rust::Args {
        lang: "rust",
        inputs: &[Path::new("../flatbuffers/event.fbs")],
        out_dir: Path::new("./src/lib/fbs"),
        ..Default::default()
    })?;

    Ok(())
}
