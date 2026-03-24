pub trait CharExtension {
    fn is_skippable(self) -> bool;
    fn is_quote(self) -> bool;
    fn is_letter(self) -> bool;
    fn is_number(self) -> bool;
    fn is_symbol(self) -> bool;
}

impl CharExtension for char {
    fn is_skippable(self) -> bool {
        self.is_whitespace()
    }

    fn is_quote(self) -> bool {
        self == '"' || self == '\''
    }

    fn is_letter(self) -> bool {
        self.is_alphabetic() || self == '_'
    }

    fn is_number(self) -> bool {
        self.is_ascii_digit()
    }

    fn is_symbol(self) -> bool {
        self.is_ascii_punctuation()
    }
}
