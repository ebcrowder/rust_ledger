use crate::error::Result;

/// returns package version
pub fn version() -> Result<()> {
    let cargo_pkg_version = option_env!("CARGO_PKG_VERSION");

    match cargo_pkg_version {
        Some(v) => println!("rust_ledger {:?}", v),
        None => println!("version not found in Cargo.toml"),
    }

    Ok(())
}
