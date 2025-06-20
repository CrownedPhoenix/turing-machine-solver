#[derive(Copy, Clone)]
pub struct Code {
    pub(crate) code: [u8; 3],
}
impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.code[0], self.code[1], self.code[2])
    }
}

#[test]
fn code_display() {
    assert_eq!(Code { code: [1, 2, 3] }.to_string(), "123")
}
