#[derive(Debug)]
pub struct Lexer<'lexer> {
    content: &'lexer [char],
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(content: &'lexer [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        // trim white space from the left
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn chop(&mut self, n: usize) -> String {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token.iter().collect::<String>()
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> String
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()));
        }

        if self.content[0].is_alphabetic() {
            return Some(
                self.chop_while(|x| x.is_alphanumeric())
                    .to_ascii_uppercase(),
            );
        }

        Some(self.chop(1))
    }
}

impl<'lexer> Iterator for Lexer<'lexer> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
