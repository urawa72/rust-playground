use std::io::Result;

fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir("generated");
    prost_build.compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}
