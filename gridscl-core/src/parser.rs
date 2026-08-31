use crate::types::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::BufRead;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SclError {
    #[error("XML Parsing Error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("Invalid SCL Format: {0}")]
    InvalidFormat(String),
}

pub struct SclParser;

impl Default for SclParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SclParser {
    pub fn new() -> Self {
        Self
    }

    /// Parses an SCL (SCD/CID/ICD) file deterministically with zero-allocation streams where possible.
    pub fn parse<R: BufRead>(&self, reader: R) -> Result<SclDocument, SclError> {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.trim_text(true);

        let mut buf = Vec::new();
        let mut ieds = Vec::new();
        let mut header = None;

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"Header" => {
                            header = Some(SclHeader {
                                id: self.get_attr(e, b"id").unwrap_or_default(),
                                version: self.get_attr(e, b"version").unwrap_or_default(),
                                revision: self.get_attr(e, b"revision").unwrap_or_default(),
                            });
                        }
                        b"IED" => {
                            let name = self.get_attr(e, b"name").unwrap_or_default();
                            ieds.push(Ied {
                                name,
                                desc: self.get_attr(e, b"desc"),
                                manufacturer: self.get_attr(e, b"manufacturer"),
                                access_points: Vec::new(),
                            });
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(SclError::XmlError(e)),
                _ => (),
            }
            buf.clear();
        }

        Ok(SclDocument {
            header: header.unwrap_or_else(|| SclHeader {
                id: "".to_string(),
                version: "".to_string(),
                revision: "".to_string(),
            }),
            ieds,
        })
    }

    fn get_attr(&self, e: &quick_xml::events::BytesStart, key: &[u8]) -> Option<String> {
        e.attributes()
            .filter_map(|a| a.ok())
            .find(|a| a.key.as_ref() == key)
            .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
    }
}
