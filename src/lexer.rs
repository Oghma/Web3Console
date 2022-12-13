//! Module for perfoming lexical analisys

/// Valid token accepted by the cli
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LexerItem<'a> {
    Address(&'a str),
    Token,
    Contract,
}

// Tokenize the command received
pub fn tokenize(command: &str) -> Result<Vec<LexerItem>, &str> {
    let words = command.split(" ");
    let mut tokens: Vec<LexerItem> = Vec::new();

    for word in words {
        let token = match word {
            "Token" => LexerItem::Token,
            "Contract" => LexerItem::Contract,
            _ if word.starts_with("0x") => LexerItem::Address(word),
            _ => return Err("Unknown command"),
        };
        tokens.push(token)
    }

    Ok(tokens)
}
