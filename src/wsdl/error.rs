use std::error::Error;
use std::fmt;
use std::io;

/// An error type for the WSDL module for convenience and easier to read function signatures
#[derive(Debug)]
pub enum WsdlError {
    /// An error occured while attempting to parse the provided document as XML
    DocumentParseError(sxd_document::parser::Error),

    /// An error that occured while trying to execute an XPath
    XPathError(sxd_xpath::Error),

    /// An I/O error, common cause is a failure to find files
    IoError(io::Error),
}

impl fmt::Display for WsdlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WsdlError::DocumentParseError(e) => write!(f, "DocumentParseError: {}", e),
            WsdlError::XPathError(e) => write!(f, "XPathError: {}", e),
            WsdlError::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl Error for WsdlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WsdlError::DocumentParseError(ref e) => Some(e),
            WsdlError::XPathError(ref e) => Some(e),
            WsdlError::IoError(ref e) => Some(e),
        }
    }
}

impl From<sxd_document::parser::Error> for WsdlError {
    fn from(e: sxd_document::parser::Error) -> Self {
        WsdlError::DocumentParseError(e)
    }
}

impl From<sxd_xpath::Error> for WsdlError {
    fn from(e: sxd_xpath::Error) -> Self {
        WsdlError::XPathError(e)
    }
}

impl From<io::Error> for WsdlError {
    fn from(e: io::Error) -> Self {
        WsdlError::IoError(e)
    }
}
