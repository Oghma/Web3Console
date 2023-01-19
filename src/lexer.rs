//! Module for perfoming lexical analisys

/// Valid token accepted by the cli
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LexerItem<'a> {
    Address(&'a str),
    Token,
    Contract,
    Abi,
}

// Tokenize the command received
pub fn tokenize(command: &str) -> Result<Vec<LexerItem>, &str> {
    let words = command.split_whitespace();
    let mut tokens: Vec<LexerItem> = Vec::new();

    for word in words {
        let token = match word {
            "Token" => LexerItem::Token,
            "Contract" => LexerItem::Contract,
            "Abi" => LexerItem::Abi,
            _ if word.starts_with("0x") => LexerItem::Address(word),
            _ => return Err("Unknown command"),
        };
        tokens.push(token)
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("Token 0x0000000000000000000000000000000000000000"),
            Ok(vec![
                LexerItem::Token,
                LexerItem::Address("0x0000000000000000000000000000000000000000")
            ])
        );

        assert_eq!(
            tokenize("0x0000000000000000000000000000000000000000").unwrap(),
            vec![LexerItem::Address(
                "0x0000000000000000000000000000000000000000"
            )]
        );

        assert_eq!(tokenize("Token").unwrap(), vec![LexerItem::Token]);
    }

    #[test]
    #[should_panic(expected = "Unknown command")]
    fn fail_uknonw_command() {
        tokenize("x0000000000000000000000000000000000000000").unwrap();
    }
}
