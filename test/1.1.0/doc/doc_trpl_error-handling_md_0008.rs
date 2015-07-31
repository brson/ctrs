fn main() {
    #[derive(Debug)]
    enum Version { Version1, Version2 }
    
    #[derive(Debug)]
    enum ParseError { InvalidHeaderLength, InvalidVersion }
    
    fn parse_version(header: &[u8]) -> Result<Version, ParseError> {
        if header.len() < 1 {
            return Err(ParseError::InvalidHeaderLength);
        }
        match header[0] {
            1 => Ok(Version::Version1),
            2 => Ok(Version::Version2),
            _ => Err(ParseError::InvalidVersion)
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
