use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum AssuanError {
    UnknownIPCCommand,
    NotImplemented,
}

impl Display for AssuanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ERR {} {} {}",
            self.assuan_code(),
            self.description(),
            self.source()
        )
    }
}

impl AssuanError {
    pub fn description(&self) -> &str {
        match self {
            AssuanError::UnknownIPCCommand => "Unknown IPC command",
            AssuanError::NotImplemented => "Not implemented",
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            AssuanError::UnknownIPCCommand => 275,
            AssuanError::NotImplemented => 69,
        }
    }

    pub fn source(&self) -> &str {
        "<User defined source 1>"
    }

    pub fn assuan_code(&self) -> u32 {
        // no idea why this should be 64 in binary,
        // their docs says SOURCE_PINENTRY is 5, but every implementation uses this
        let source_bits: u32 = 0b1000000; // 7 bits
        let reserved_bits: u32 = 0b0000000; // seems to be 7 bits of zeros
        let error_code: u32 = self.code(); // 16 bits from the `code` function

        let assuan_code = (source_bits << 23) | (reserved_bits << 16) | error_code;

        assuan_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_ipc_error_has_correct_code_and_description() {
        assert_eq!(AssuanError::UnknownIPCCommand.code(), 275);
        assert_eq!(AssuanError::UnknownIPCCommand.assuan_code(), 536871187);
        assert_eq!(
            AssuanError::UnknownIPCCommand.description(),
            "Unknown IPC command"
        );
        assert_eq!(
            AssuanError::UnknownIPCCommand.source(),
            "<User defined source 1>"
        );
        assert_eq!(
            AssuanError::UnknownIPCCommand.to_string(),
            "ERR 536871187 Unknown IPC command <User defined source 1>"
        );
    }

    #[test]
    fn not_implemented_error_has_correct_code_and_description() {
        assert_eq!(AssuanError::NotImplemented.code(), 69);
        assert_eq!(AssuanError::NotImplemented.assuan_code(), 536870981);
        assert_eq!(AssuanError::NotImplemented.description(), "Not implemented");
        assert_eq!(
            AssuanError::NotImplemented.source(),
            "<User defined source 1>"
        );
        assert_eq!(
            AssuanError::NotImplemented.to_string(),
            "ERR 536870981 Not implemented <User defined source 1>"
        );
    }
}
