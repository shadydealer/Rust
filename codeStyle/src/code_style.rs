use std::str::FromStr;
use std;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CodeStyle {
    Underscored,
    Camelcased,
    ScreamingSnakecased,
    Unknown,
    Mixed
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CodeStyleError {
    EmptyString,
    InvalidFirstChar(char),
    InvalidChar(char),
}

impl FromStr for CodeStyle {
    type Err = CodeStyleError;

    fn from_str(identifier: &str) -> Result<CodeStyle, CodeStyleError> {
        
        fn identifier_code_style(mut iter:std::str::Chars,
                                 mut style: CodeStyle,
                                 mut error:Option<CodeStyleError>)
                                 -> Result<CodeStyle, CodeStyleError> {

            if !error.is_none() {
                return Err(error.unwrap());
            }

            if iter.as_str() == "" {
                return Ok(style);
            }else{
                let c = iter.next().unwrap();
                
                if c == '_' {
                    if style == CodeStyle::Unknown {
                        style = CodeStyle::Underscored;
                    }else if style == CodeStyle::Camelcased {
                        style = CodeStyle::Mixed;
                    }
                }else if char::is_alphabetic(c) {
                    if char::is_uppercase(c) {
                        if style == CodeStyle::Underscored {
                            style = CodeStyle::Mixed;
                        }else if style == CodeStyle::Unknown {
                            style = CodeStyle::Camelcased;
                        }
                    }else {
                        if style == CodeStyle::ScreamingSnakecased {
                            style = CodeStyle::Camelcased;
                        }
                    }
                }else if !char::is_numeric(c){
                    error = Some(CodeStyleError::InvalidChar(c));
                }
                identifier_code_style(iter, style, error)
            }
        }

        let trimmed_identifier        = String::from(identifier.trim());
        let mut char_iter             = trimmed_identifier.chars();
        let first_char:Option<char>   = char_iter.next();
        let mut code_style: CodeStyle = CodeStyle::Unknown;
        
        let mut error: Option<CodeStyleError> = None;

        if first_char.is_none() {
            error = Some(CodeStyleError::EmptyString);
        } else {
            if !char::is_alphabetic(first_char.unwrap()) {
                error = Some(CodeStyleError::InvalidFirstChar(first_char.unwrap()));
            }
            code_style = if char::is_uppercase(first_char.unwrap()) {
                                    CodeStyle::ScreamingSnakecased
                                }else{
                                    CodeStyle::Unknown
                                };
        }

        identifier_code_style(char_iter, code_style, error)   
    }
}

#[test]
fn is_valid() {
    assert_eq!(CodeStyle::from_str("кирилицата").ok(),  Some(CodeStyle::Unknown));
    assert_eq!(CodeStyle::from_str("im").ok(),          Some(CodeStyle::Unknown));
    assert_eq!(CodeStyle::from_str("  so  ").ok(),      Some(CodeStyle::Unknown));
    assert_eq!(CodeStyle::from_str("ti_red").ok(),      Some(CodeStyle::Underscored));
    assert_eq!(CodeStyle::from_str("of_Your").ok(),     Some(CodeStyle::Mixed));
    assert_eq!(CodeStyle::from_str("Shit").ok(),        Some(CodeStyle::Camelcased));
    assert_eq!(CodeStyle::from_str("GREG").ok(),        Some(CodeStyle::ScreamingSnakecased));
 }

#[test]
fn is_invalid() {
    assert_eq!(CodeStyle::from_str("1MyMommaToldMe").err(), Some(CodeStyleError::InvalidFirstChar('1')));
    assert_eq!(CodeStyle::from_str("I was").err(), Some(CodeStyleError::InvalidChar(' ')));
    assert_eq!(CodeStyle::from_str("*special").err(), Some(CodeStyleError::InvalidFirstChar('*')));
    assert_eq!(CodeStyle::from_str("").err(), Some(CodeStyleError::EmptyString));
}
