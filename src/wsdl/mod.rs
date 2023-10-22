//! Provides WSDL support for the crate
//! This is used to generate descriptions of a SOAP API

use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;
use std::path::Path;
use sxd_document::parser::parse;
use sxd_document::Package;
use sxd_xpath::nodeset::Node;
use sxd_xpath::{evaluate_xpath, Value};

mod error;

#[doc(no_inline)]
pub use error::WsdlError;

/// A structure to describe what services are available according to the provided WSDL document
pub struct WsdlDocument {
    raw: String,
    package: Option<Package>,

    /// Stores if the document has been parsed
    pub parsed: bool,

    /// Stores the WSDL version
    /// Note: until the document has been parsed, there is no guarentee that the value stored in
    /// this field is correct
    pub version: WsdlVersion,

    /// A collection of all the types specified in the WSDL document
    pub types: HashMap<String, WsdlType>,
}

/// Represents all (hopefully) supported WSDL versions
#[derive(Debug, PartialEq, Eq)]
pub enum WsdlVersion {
    /// WSDL Version 1.1
    Wsdl11,
    /// WSDL Version 2.0 (aka 1.2)
    Wsdl20,
}

/// Represents a WSDL type definition
#[derive(Debug, PartialEq, Eq)]
pub struct WsdlType {
    name: String,
}

impl fmt::Debug for WsdlDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WsdlDocument")
            .field("raw", &format_args!("{} characters", self.raw.len()))
            .field("parsed", &self.parsed)
            .field("package", &self.package)
            .field("version", &self.version)
            .field("types", &self.types)
            .finish()
    }
}

impl WsdlDocument {
    /// Creates a new WsdlDocument structure by reading the contents the file at the provided path
    ///
    /// # Example
    /// ```rust
    /// let api_description = WsdlDocument::new(path_to_my_file);
    /// ```
    pub fn new(path: &Path) -> Result<Self, WsdlError> {
        Ok(Self {
            raw: read_to_string(path)?,
            package: None,
            parsed: false,
            version: WsdlVersion::Wsdl11,
            types: HashMap::new(),
        })
    }

    /// Parses the WSDL document the structure was created with and fills out the structure's
    /// fields to the best of its ability. This is a necessary step in generating the description
    /// of a SOAP API and should be called almost always after creating a new structure.
    ///
    /// Some special behavior should be noted:
    /// - When determining the WSDL document's version, if there is an error or an unsupported
    ///   is found, the parser will fall back onto version (WsdlVersion::Wsdl11)[1.1]
    ///
    /// # Example
    /// ```rust
    /// let api_description = WsdlDocument::new(path_to_my_file);
    /// if let Err(e) = api_description.parse() {
    ///     eprintln!("Failed to parse WSDL: {}", e);
    /// }
    /// ```
    pub fn parse(&mut self) -> Result<(), WsdlError> {
        self.parsed = true;

        self.package = Some(parse(self.raw.as_str())?);
        let document = self.package.as_ref().unwrap().as_document();

        // Attempt to detect WSDL version, falling back on 1.1 if there is an error
        if let Ok(Value::Nodeset(nodes)) = evaluate_xpath(&document, "/*") {
            for node in nodes.iter().filter(|e| match e {
                Node::Element(_) => true, // We only care about element nodes
                _ => false,
            }) {
                let element = node.element().unwrap();
                if element.name().local_part() == "description" {
                    self.version = WsdlVersion::Wsdl20;
                }
            }
        }

        Ok(())
    }
}
