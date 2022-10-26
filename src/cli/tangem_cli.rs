use anyhow::Result;
use tokio::task::spawn_blocking;

use crate::cli::cli_error::CliError;

pub struct TangemCli {
    // sdk: Option<TangemSdk>
    // response_converter: MoshiJsonConverter.INSTANCE
    // card: Option<Card>
}

impl TangemCli {
    pub fn execute(&self, command: TangemCliCommand) {
        if let Some(sdk) = self.sdk {
            // TODO
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TangemCliCommand {
    Read,
    Sign,
    ReadFiles,
    WriteFiles,
    DeleteFiles,
    CreateWallet,
    PurgeWallet
}

impl TryFrom<&str> for TangemCliCommand {
    type Error = CliError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "read" => Ok(TangemCliCommand::Read),
            "sign" => Ok(TangemCliCommand::Sign),
            "readfiles" => Ok(TangemCliCommand::ReadFiles),
            "writefiles" => Ok(TangemCliCommand::WriteFiles),
            "deletefiles" => Ok(TangemCliCommand::DeleteFiles),
            "createwallet" => Ok(TangemCliCommand::CreateWallet),
            "purgewwallet" => Ok(TangemCliCommand::PurgeWallet),
            _ => Err(Self::Error::CommandNotFound { command: value.to_string() }.into())
        }
    }
}


#[cfg(test)]
mod test {
    use crate::cli::tangem_cli::TangemCliCommand;

    #[test]
    fn test_from() {
        assert_eq!(TangemCliCommand::try_from("createwallet").unwrap(), TangemCliCommand::CreateWallet)
    }
}