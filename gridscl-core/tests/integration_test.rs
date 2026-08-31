use gridscl_core::{SclParser, SclValidator};
use std::io::Cursor;
use std::time::Instant;

#[test]
fn test_parser_and_memory() {
    let mock_scd = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <SCL xmlns="http://www.iec.ch/61850/2003/SCL" version="2007" revision="B">
        <Header id="TestProject" version="1" revision="1"/>
        <IED name="IED1" desc="Test IED" manufacturer="VendorA">
            <AccessPoint name="AP1">
                <Server>
                    <LDevice inst="LD0">
                        <LN0 lnClass="LLN0" inst="" lnType="LLN0_Type"/>
                    </LDevice>
                </Server>
            </AccessPoint>
        </IED>
    </SCL>
    "#;

    let parser = SclParser::new();
    let reader = Cursor::new(mock_scd.as_bytes());

    let start = Instant::now();
    let doc = parser.parse(reader).expect("Failed to parse mock SCD");
    let duration = start.elapsed();

    // Verify sub-50ms execution speed
    assert!(
        duration.as_millis() < 50,
        "Parsing took too long: {:?}",
        duration
    );

    // Verify parsed structure
    assert_eq!(doc.header.id, "TestProject");
    assert_eq!(doc.ieds.len(), 1);
    assert_eq!(doc.ieds[0].name, "IED1");

    // Validate
    let validator = SclValidator::new();
    let result = validator.validate(&doc);
    assert!(result.is_valid);
}
