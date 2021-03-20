/// Convenience wrapper around string allowing for constant character-based access.
pub struct CharString<'a> {
    string: &'a str,
    char2byte: Vec<usize>,
}

impl<'a> CharString<'a> {
    pub fn new(string: &'a str) -> CharString {
        let mut char2byte: Vec<usize> = string.char_indices().map(|x| x.0).collect();
        char2byte.push(string.len());

        CharString { string, char2byte }
    }

    pub fn substr(&self, s: usize, e: usize) -> &str {
        &self.string[self.char2byte[s]..self.char2byte[e]]
    }

    pub fn prefix(&self, e: usize) -> &str {
        &self.string[..self.char2byte[e]]
    }

    pub fn suffix(&self, s: usize) -> &str {
        &self.string[self.char2byte[s]..]
    }

    pub fn len(&self) -> usize {
        self.char2byte.len() - 1
    }

    pub fn num_bytes(&self) -> usize {
        self.string.len()
    }

    pub fn char2byte(&self, i: usize) -> usize {
        self.char2byte[i]
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::*;

    #[test]
    fn take_substrings() {
        let string = "Tür, Straße, Москва";
        let char_string = CharString::new(&string);

        assert_eq!(char_string.substr(0, 3), "Tür");
        assert_eq!(char_string.substr(5, 11), "Straße");
        assert_eq!(char_string.substr(13, 19), "Москва");
    }

    #[test]
    fn take_prefixes() {
        let string = "Tür, Straße, Москва";
        let char_string = CharString::new(&string);

        assert_eq!(char_string.prefix(11), "Tür, Straße");
        assert_eq!(char_string.prefix(19), "Tür, Straße, Москва");
        assert_eq!(char_string.prefix(0), "");
    }

    #[test]
    fn take_suffixes() {
        let string = "Tür, Straße, Москва";
        let char_string = CharString::new(&string);

        assert_eq!(char_string.suffix(11), ", Москва");
        assert_eq!(char_string.suffix(19), "");
        assert_eq!(char_string.suffix(0), "Tür, Straße, Москва");
    }

    #[test]
    fn get_length() {
        let string = "öüä";
        let char_string = CharString::new(&string);

        assert_eq!(char_string.len(), 3);
    }
}
