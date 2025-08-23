use logviewer::core::{models::LogLevel, parser::{LogParser, ShortLogParser}};

#[test]
fn test_parse_line() {
    let json = r#"
    {
        "t": "2024-01-01 00:00:00.000+0000",
        "l": "INFO",
        "mt": "Ok",
        "lg": "mylogger"
    }
    "#;

    let parser = ShortLogParser;
    let entry = parser.parse_line(json).unwrap();

    assert_eq!(entry.level.unwrap(), LogLevel::Info);
    assert_eq!(entry.message.unwrap(), "Ok");
}
