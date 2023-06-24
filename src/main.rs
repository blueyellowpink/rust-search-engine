use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Lexer<'lexer> {
    content: &'lexer [char],
}

impl<'lexer> Lexer<'lexer> {
    fn new(content: &'lexer [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        // trim white space from the left
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn chop(&mut self, n: usize) -> &'lexer [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'lexer [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    fn next_token(&mut self) -> Option<&'lexer [char]> {
        self.trim_left();
        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()));
        }

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphanumeric()));
        }

        Some(self.chop(1))
    }
}

impl<'lexer> Iterator for Lexer<'lexer> {
    type Item = &'lexer [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn read_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = fs::File::open(file_path)?;
    let event_reader = EventReader::new(file);
    let mut content = String::new();
    for event in event_reader.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
            content.push_str(" ");
        }
    }
    Ok(content)
}

type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

fn main() {
    let dir_path = "docs.gl/test";
    let dirs = fs::read_dir(dir_path).unwrap();
    let mut tf_index = TermFreqIndex::new();
    for file in dirs {
        let file_path = file.unwrap().path();
        let content = read_xml_file(&file_path)
            .unwrap()
            .chars()
            .collect::<Vec<_>>();

        println!("Indexing {file_path:?}");

        let mut tf = TermFreq::new();

        for token in Lexer::new(&content) {
            let term = token
                .iter()
                .map(|x| x.to_ascii_uppercase())
                .collect::<String>();

            tf.entry(term).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut stats = tf.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();

        tf_index.insert(file_path, tf);
    }

    for (path, tf) in tf_index {
        println!("{path:?} has {count} unique indexes", count = tf.len());
    }
}
