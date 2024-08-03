use std::str::FromStr;

use thiserror::Error;



pub enum PromptIntent {
    Quit,
    Run,
    Step(usize),
    Goto(u16),
    PrintRange(u16, u16),
    PrintZeroPage,
    PrintContext,
    None,
}

#[derive(Error, Debug)]
pub enum IntentParseError {
    #[error("Malformed range start: {0}")]
    MalformedRangeStart(String),
    #[error("Malformed range end: {0}")]
    MalformedRangeEnd(String),
    #[error("Malformed goto address: {0}")]
    MalformedGoto(String),
}
impl FromStr for PromptIntent {
    type Err = IntentParseError;

    fn from_str(value: &str) -> Result<Self, IntentParseError> {
        if value == "q" {
            Ok(PromptIntent::Quit)
        } else if value == "r" {
            Ok(PromptIntent::Run)
        } else if value == "p" {
            Ok(PromptIntent::PrintContext)
        } else if value == "zp" {
            Ok(PromptIntent::PrintZeroPage)  
        } else if value.starts_with("$") {
            if let Some((lhs, rhs)) = value.split_once(":") {
                let start = u16::from_str_radix(&lhs[1..], 16)
                    .map_err(|_err| IntentParseError::MalformedRangeStart(lhs.to_string()))?;

                let end = if rhs.starts_with("+") {
                    start + u16::from_str_radix(&rhs[1..], 10)
                        .map_err(|_err| IntentParseError::MalformedRangeEnd(rhs.to_string()))?
                } else {
                    u16::from_str_radix(&rhs, 16)
                        .map_err(|_err| IntentParseError::MalformedRangeStart(lhs.to_string()))?
                };

                Ok(PromptIntent::PrintRange(start, end))
            } else {
                if let Ok(start) = u16::from_str_radix(&value[1..], 16) {
                    Ok(PromptIntent::PrintRange(start, start + 1))
                } else {
                    Err(IntentParseError::MalformedRangeStart(value.to_string()))
                }
            }
        } else if value.starts_with(">") {
            let addr = u16::from_str_radix(&value[1..], 16)
                .map_err(|_err| IntentParseError::MalformedGoto(value.to_string()))?;
            Ok(PromptIntent::Goto(addr))
        } else if let Ok(step) = usize::from_str(&value) {
            Ok(PromptIntent::Step(step))
        } else {
            Ok(PromptIntent::None)
        }
    }
}

