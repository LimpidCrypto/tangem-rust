use std::ffi::CStr;

use anyhow::{anyhow, Result};
use pcsc::{Card, Context, Disposition, MAX_BUFFER_SIZE, Protocols, ShareMode};
use crate::reader::reader_error::ReaderError;


pub struct SmartCardReader {
    card: Option<Card>
}

impl SmartCardReader {
    pub fn connect( context: &Context, reader: &CStr ) -> Result<Self> {
        let card = context.connect(
            reader,
            ShareMode::Shared,
            Protocols::ANY
        )?;
        Ok( Self { card: Some(card) } )
    }

    pub fn disconnect( self ) -> Result<()> {
        if let Some(card) = self.card {
            return match card.disconnect(Disposition::EjectCard) {
                Ok(_) => Ok(()),
                Err((_, err)) => Err(err.into())
            };
        }

        Ok(())
    }

    pub fn transceive_apdu( &self, apdu: &[u8] ) -> Result<Vec<u8>> {
        if let Some(card) = &self.card {
            let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
            let rapdu = card.transmit(apdu, &mut rapdu_buf)?
                .to_owned();

            return Ok(rapdu);
        }

        Err( ReaderError::NoCardConnected.into() )
    }
}