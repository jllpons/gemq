use std::io::{self, BufRead, BufReader, Read};

use anyhow::Result;
use nom::IResult;
use nom::bytes::complete::tag;


struct Locus {
    name: String,
    length: u32,
    molecule_type: String,
    genbank_division: String,
    modification_date: String,
}

pub struct GenbankRecord {
    locus: Locus,
}

impl Locus {
    pub fn from_line_iter<I>(lines_iter: &mut I) -> Result<Self>
    where
        I: Iterator<Item = io::Result<String>>,
    {
        let line = lines_iter.next().ok_or_else(|| {
            anyhow::anyhow!("Unexpected end of file while parsing LOCUS")
        })??;

        let locus = Locus {
            name: line[12..24].trim().to_string(),
            length: line[29..41].trim().parse()?,
            mol_type: line[47..53].trim().to_string(),
            division: line[62..65].trim().to_string(),
            date: line[68..79].trim().to_string(),
        };

        Ok(locus)
    }

    fn parse_locus(input: &str) -> IResult<&str, Locus> {
        let (leftover_input, _) = tag("LOCUS")(input)?;

        let locus = Locus {
            name: name.trim().to_string(),
            length: length.trim().parse().unwrap(),
            mol_type: mol_type.trim().to_string(),
            division: division.trim().to_string(),
            date: date.trim().to_string(),
        };

        Ok((input, locus))
    }
}

impl GenbankRecord {
    // Parser data from a reader and returns a GenbankRecord
    pub fn try_from_reader<R: Read>(reader: R) -> Result<Self> {
        let buf_reader = BufReader::new(reader);
        let mut lines_iter = buf_reader.lines();

        // While there are lines to read, print them.
        while let Some(line) = lines_iter.next() {
            let line = line?;

            // If line is "//" then we have reached the end of the record

            let locus = Locus::from_line_iter(&mut lines_iter)?;
        }

        unimplemented!();
        Ok(gb_record)
    }
}
