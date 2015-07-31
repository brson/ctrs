// http://rosettacode.org/wiki/FASTA_format
// Ported and adapted from rosettacode D example
use std::fs::File;
use std::io::{BufReader, BufRead};

// We use a type parameter bound `<T: Buffer>` to accept all kinds of buffers
fn format_fasta<T: BufRead>(reader: &mut T) -> String {
    reader.lines().map(|l| l.unwrap()).fold(String::new(), |mut out, line| {
        // We need to trim new lines
        let ln = line.trim();

        // Lines that begin with '>' require special treatment
        match &ln[..1] {
            ">" => {
                if out.len() > 0 {
                    out.push('\n');
                }

                // Push skipping the '>'
                out.push_str(&ln[1..]);
                out.push_str(": ");
            }
            // Other lines are just pushed
            _ => out.push_str(ln)
        }
        out
    })
}

fn read_file() -> String {
    let file = File::open("src/resources/test_data.fasta").unwrap();
    format_fasta(&mut BufReader::new(file))
}

#[cfg(not(test))]
fn main() {
    let s = read_file();
    println!("{}", s);
}

#[test]
fn test_format_fasta() {
    let s = read_file();
    assert_eq!(s, "Rosetta_Example_1: THERECANBENOSPACE
Rosetta_Example_2: THERECANBESEVERALLINESBUTTHEYALLMUSTBECONCATENATED");
}
