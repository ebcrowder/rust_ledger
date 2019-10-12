// returns all general ledger transactions

extern crate serde_yaml;

pub fn register_yaml_test(filename: &str) -> Result<(), std::io::Error> {
    std::fs::File::open("something.yaml")?;
    let d: String = serde_yaml::from_reader(f)?;
    println!("Read YAML string: {}", d);

    Ok(())
}
