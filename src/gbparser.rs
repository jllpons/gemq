use std::io::{self, BufRead, BufReader, Read};
use std::str::from_utf8;

use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alphanumeric1, i32, space1};
use nom::sequence::tuple;
use nom::IResult;
use nom::Needed;

use crate::genbank::{GenbankRecord, Locus, Molecule};

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB

#[derive(Debug, Clone, PartialEq)]
enum State {
    Start,
    Locus,
    Partial,
    End,
}

struct GenbankParser {
    state: State,
    previous_state: State,
    buffer: Vec<u8>,
    leftover: Vec<u8>,
    reader: Box<dyn BufRead>,
    record: GenbankRecord,
}

enum GenbankField {
    Locus(Locus),
    Partial,
    Missing,
    End,
}

fn state_machine(genbank_parser: &mut GenbankParser) -> Result<GenbankField> {
    match genbank_parser.state {
        State::Start => {
            if !genbank_parser.buffer.starts_with(b"LOCUS") {
                // Print the first line of the file
                let first_line = from_utf8(&genbank_parser.buffer).unwrap();
                println!("First line: {}", first_line);
                println!("No LOCUS field found");
                genbank_parser.state = State::Locus;

                return Ok(GenbankField::Missing);
            }

            let result = parse_locus(&genbank_parser.buffer);

            match result {
                Ok((remaining, locus)) => {
                    genbank_parser.state = State::Locus;
                    genbank_parser.buffer = remaining.to_vec();
                    return Ok(GenbankField::Locus(locus));
                }
                Err(nom::Err::Incomplete(Needed::Size(_))) => {
                    genbank_parser.state = State::Partial;
                    genbank_parser.leftover = genbank_parser.buffer.to_vec();
                    return Ok(GenbankField::Partial);
                }
                Err(e) => {
                    genbank_parser.state = State::End;
                    return Err(anyhow::anyhow!("Error parsing LOCUS: {:?}", e));
                }
            }
        }

        State::Locus => {
            return Ok(GenbankField::End);
        }

        State::Partial => {
            let mut buffer = vec![0; CHUNK_SIZE];
            let bytes_read = genbank_parser.reader.read(&mut buffer)?;
            genbank_parser.buffer = [&genbank_parser.leftover[..], &buffer[..bytes_read]].concat();
            genbank_parser.leftover = Vec::new();
            genbank_parser.state = genbank_parser.previous_state.clone();
            return Ok(GenbankField::Partial);
        }

        State::End => {
            return Ok(GenbankField::End);
        }
    }
}


fn parse_locus(chunk: &[u8]) -> IResult<&[u8], Locus> {
    let (chunk, _) = tag("LOCUS")(chunk)?;

    let (chunk, value) = take_until("\n")(chunk)?;

    let (
        _,
        (_, name, _, length, _, _, _, molecule_type, _, molecule_kind, _, genbank_division, _, modification_date),
    ) = tuple((
        space1,
        alphanumeric1,
        space1,
        i32,
        space1,
        alphanumeric1,
        space1,
        alphanumeric1,
        space1,
        alphanumeric1,
        space1,
        alphanumeric1,
        space1,
        alphanumeric1,
    ))(value)?;

    let locus = Locus {
        name: String::from_utf8(name.to_vec()).unwrap(),
        length: length,
        molecule: Molecule {
            type_: String::from_utf8(molecule_type.to_vec()).unwrap(),
            kind: String::from_utf8(molecule_kind.to_vec()).unwrap(),
        },
        genbank_division: String::from_utf8(genbank_division.to_vec()).unwrap(),
        modification_date: String::from_utf8(modification_date.to_vec()).unwrap(),
    };

    Ok((chunk, locus))
}

pub fn parse_gb(buf_reader: Box<dyn BufRead>) -> Result<GenbankRecord> {
    let mut gb_parser = GenbankParser {
        state: State::Start,
        previous_state: State::Start,
        buffer: vec![0; CHUNK_SIZE],
        leftover: Vec::new(),
        reader: buf_reader,
        record: GenbankRecord::new(),
    };

    gb_parser.reader.read(&mut gb_parser.buffer)?;

    while gb_parser.state != State::End {
        if gb_parser.buffer.is_empty() || gb_parser.buffer.starts_with(b"//") {
            gb_parser.state = State::End;
        }

        let field = state_machine(&mut gb_parser)?;
        match field {
            GenbankField::Locus(locus) => {
                gb_parser.record.locus = Some(locus);
            }
            GenbankField::Partial => {
                continue;
            }
            GenbankField::Missing => {
                continue;
            }
            GenbankField::End => {
                break;
            }
        }
    }

    Ok(gb_parser.record)
}
