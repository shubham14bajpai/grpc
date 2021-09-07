fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/v1/storage.proto")?;
    tonic_build::compile_protos("proto/v2/storage.proto")?;
    Ok(())
}
