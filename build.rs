fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/cadastro.proto")?;
    tonic_build::compile_protos("proto/biblioteca.proto")?;
    Ok(())
}
