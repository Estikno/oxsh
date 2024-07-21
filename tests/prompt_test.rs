use std::io::Cursor;
use oxsh::prompt::read_input;

#[test]
fn test_read_input_with_special_chars() {
    // Arrange
    let input = "Hello\tWorld\r\n";
    let mut reader = Cursor::new(input);

    // Act
    let result = read_input(&mut reader);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello\tWorld\r\n");
}

#[test]
fn test_read_input_with_empty_line() {
    // Arrange
    let input = "\n";
    let mut reader = Cursor::new(input);

    // Act
    let result = read_input(&mut reader);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\n");
}

#[test]
fn test_read_input_with_only_special_chars() {
    // Arrange
    let input = "\t\r\n";
    let mut reader = Cursor::new(input);

    // Act
    let result = read_input(&mut reader);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "\t\r\n");
}

#[test]
fn test_read_input_with_no_input() {
    // Arrange
    let input = "";
    let mut reader = Cursor::new(input);

    // Act
    let result = read_input(&mut reader);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_read_input_with_long_input() {
    // Arrange
    let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n";
    let mut reader = Cursor::new(input);

    // Act
    let result = read_input(&mut reader);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n");
}