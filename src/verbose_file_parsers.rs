use std::{io::BufRead, path::Path};

use crate::file_parsers::GzBufReader;
use crate::graphmol::rw_mol::RWMol;
use flate2::bufread::GzDecoder;

pub struct VerboseMolBlockIter<R: BufRead> {
    buf_read: R,
    buf: Vec<u8>,
    sanitize: bool,
    remove_hs: bool,
    strict_parsing: bool,
}

impl<R: BufRead> VerboseMolBlockIter<R> {
    pub fn new(buf_read: R, sanitize: bool, remove_hs: bool, strict_parsing: bool) -> Self {
        VerboseMolBlockIter {
            buf_read,
            buf: Vec::with_capacity(1024),
            sanitize,
            remove_hs,
            strict_parsing,
        }
    }
}

impl VerboseMolBlockIter<GzBufReader> {
    pub fn from_gz_file(
        p: impl AsRef<Path>,
        sanitize: bool,
        remove_hs: bool,
        strict_parsing: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let path = p.as_ref().to_owned();

        let file = std::fs::File::open(path).unwrap();
        let buf_reader = std::io::BufReader::new(file);

        let gz_decoder = GzDecoder::new(buf_reader);
        let gz_buf_reader = std::io::BufReader::new(gz_decoder);

        Ok(Self::new(
            gz_buf_reader,
            sanitize,
            remove_hs,
            strict_parsing,
        ))
    }
}

impl<R: BufRead> Iterator for VerboseMolBlockIter<R> {
    type Item = Result<(RWMol, String), String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Consume all `$` characters and break when buffer is larger than 1. Exit
        // function at EOF.
        loop {
            self.buf.clear();
            let read = self.buf_read.read_until(b'$', &mut self.buf).unwrap();
            if read == 0 {
                return None;
            } else if read == 1 {
                continue;
            } else {
                break;
            }
        }

        let block = std::str::from_utf8(&self.buf).unwrap();
        let block = block.trim();

        let rw_mol =
            RWMol::from_mol_block(block, self.sanitize, self.remove_hs, self.strict_parsing);

        let result = match rw_mol {
            Some(rw_mol) => Ok((rw_mol, block.to_string())),
            _ => Err(block.to_string()),
        };

        Some(result)
    }
}
