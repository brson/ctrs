fn main() {
    #[derive(Debug)]
    enum Version { Version1, Version2 }
    
    fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
    match header.get(0) {
    None => Err("invalid header length"),
    Some(&1) => Ok(Version::Version1),
    Some(&2) => Ok(Version::Version2),
    Some(_) => Err("invalid version")
    }
    }
    
    let version = parse_version(&[1, 2, 3, 4]);
    match version {
    Ok(v) => println!("working with version: {:?}", v),
    Err(e) => println!("error parsing header: {:?}", e),
    }
}
