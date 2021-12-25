fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/plugins.proto");
    tonic_build::configure().compile(&["../proto/plugins.proto"], &["../proto"])?;
    Ok(())
}
