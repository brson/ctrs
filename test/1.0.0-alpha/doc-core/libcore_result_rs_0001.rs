fn main() {
    #[derive(Show)]
    enum Version { Version1, Version2 }
    
    fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
        if header.len() < 1 {
            return Err("invalid header length");
        }
        match header[0] {
            1 => Ok(Version::Version1),
            2 => Ok(Version::Version2),
            _ => Err("invalid version")
        }
    }
    
    let version = parse_version(&[1, 2, 3, 4]);
    match version {
        Ok(v) => {
            println!("working with version: {:?}", v);
        }
        Err(e) => {
            println!("error parsing header: {:?}", e);
        }
    }
}
