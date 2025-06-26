#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Code {
    pub(crate) code: [u8; 3],
}

impl IntoIterator for Code {
    type Item = u8;
    // TODO(perf,mem): Implement custom Iterator
    type IntoIter = std::array::IntoIter<u8, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.code.into_iter()
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.blue(), self.yellow(), self.purple())
    }
}

impl Code {
    pub fn iter(&self) -> <Code as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn blue(&self) -> u8 {
        self.code[0]
    }
    pub fn yellow(&self) -> u8 {
        self.code[1]
    }
    pub fn purple(&self) -> u8 {
        self.code[2]
    }
}

#[test]
fn code_display() {
    assert_eq!(Code { code: [1, 2, 3] }.to_string(), "123")
}
