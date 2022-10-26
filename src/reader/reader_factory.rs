use anyhow::{Result};
use pcsc::{Context, Scope};
use crate::reader::reader_error::ReaderError;
use crate::reader::smart_card_reader::SmartCardReader;


pub struct ReaderFactory {
    index_of_terminal: usize
}

impl Default for ReaderFactory {
    fn default() -> Self {
        Self {
            index_of_terminal: 0
        }
    }
}

impl ReaderFactory {
    pub fn create(self) -> Result<SmartCardReader> {
        let scope = Scope::Terminal;
        let context = Context::establish( scope )?;

        let mut readers_buf = [0; 2048];
        let readers = context.list_readers( &mut readers_buf )?;

        // find reader by index
        let reader = match readers.into_iter().enumerate().find(|&i| i.0 == self.index_of_terminal) {
            Some(reader) => { reader.1 },
            None => {
                return Err( ReaderError::ReaderNotFound { index: self.index_of_terminal }.into() );
            }
        };

        //connect to card
        let card = SmartCardReader::connect(&context, reader)?;

        Ok(card)
    }
}


#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use crate::reader::reader_factory::ReaderFactory;

    #[test]
    fn test_card() {
        let apdu = b"\x00\xA4\x04\x00\x0A\xA0\x00\x00\x00\x62\x03\x01\x0C\x06\x01";
        let smart_card = ReaderFactory { index_of_terminal: 0 }.create().unwrap();
        let rapdu = smart_card.transceive_apdu(apdu).unwrap();
        let text = &rapdu[0 .. rapdu.len()-2];
        println!("{}", std::str::from_utf8(&text).unwrap());
    }
}