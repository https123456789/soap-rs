use soap_rs::wsdl::{WsdlDocument, WsdlVersion};
use std::error::Error;
use std::path::Path;

/// Can the WSDL module successfully parse valid inputs
#[test]
fn wsdl_parse_is_successful() -> Result<(), Box<dyn Error>> {
    let file_paths = [
        format!("{}/test-data/wsdl_sample_1.xml", env!("CARGO_MANIFEST_DIR")),
        format!("{}/test-data/wsdl_sample_2.xml", env!("CARGO_MANIFEST_DIR")),
    ];

    for file_path in file_paths {
        let mut document = WsdlDocument::new(Path::new(file_path.as_str()))?;
        document.parse()?;
        assert!(document.parsed);
    }

    Ok(())
}

/// Can the WSDL module detect the WSDL 1.1 version
#[test]
fn wsdl_can_detect_11() -> Result<(), Box<dyn Error>> {
    let file_path = format!("{}/test-data/wsdl_sample_1.xml", env!("CARGO_MANIFEST_DIR"));
    let mut document = WsdlDocument::new(Path::new(file_path.as_str()))?;

    document.parse()?;
    assert_eq!(document.version, WsdlVersion::Wsdl11);

    Ok(())
}

/// Can the WSDL module detect the WSDL 2.0 version
#[test]
fn wsdl_can_detect_20() -> Result<(), Box<dyn Error>> {
    let file_path = format!("{}/test-data/wsdl_sample_2.xml", env!("CARGO_MANIFEST_DIR"));
    let mut document = WsdlDocument::new(Path::new(file_path.as_str()))?;

    document.parse()?;
    assert_eq!(document.version, WsdlVersion::Wsdl20);

    Ok(())
}
