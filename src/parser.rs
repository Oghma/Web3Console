//! Module for performing parsing over the tokens generated by a lexer

use crate::lexer::LexerItem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Address<'a> {
    pub address: &'a str,
}

impl<'a> Address<'a> {
    pub fn new(address: &'a str) -> Self {
        Self { address }
    }
}

impl<'a> From<LexerItem<'a>> for Address<'a> {
    fn from(item: LexerItem<'a>) -> Self {
        match item {
            LexerItem::Address(addr) => Self { address: addr },
            _ => panic!("Wrong lexer item"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command<'a> {
    Token { address: Address<'a> },
    Contract { address: Address<'a> },
}

pub fn parse<'a, 'b>(tokens: &Vec<LexerItem<'a>>) -> Result<Command<'a>, &'b str> {
    parse_command(tokens)
}

fn parse_command<'a, 'b>(tokens: &[LexerItem<'a>]) -> Result<Command<'a>, &'b str> {
    match tokens[0] {
        LexerItem::Token => Ok(Command::Token {
            address: parse_address(&tokens[1..])?,
        }),
        LexerItem::Contract => Err("Not yet supported"),
        _ => Err("Unknown command"),
    }
}

fn parse_address<'a, 'b>(tokens: &[LexerItem<'a>]) -> Result<Address<'a>, &'b str> {
    if tokens.is_empty() {
        return Err("Missing address");
    }
    Ok(Address::from(tokens[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(&vec![
                LexerItem::Token,
                LexerItem::Address("0x0000000000000000000000000000000000000000"),
            ]),
            Ok(Command::Token {
                address: Address::new("0x0000000000000000000000000000000000000000")
            })
        );
    }

    #[test]
    #[should_panic(expected = "Missing address")]
    fn fail_missing_address() {
        parse(&vec![LexerItem::Token]).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unknown command")]
    fn fail_unknown_command() {
        parse(&vec![LexerItem::Address(
            "0x0000000000000000000000000000000000000000",
        )])
        .unwrap();
    }
}