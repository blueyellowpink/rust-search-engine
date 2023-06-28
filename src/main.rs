use std::{error::Error, fs, io, path::Path, process::ExitCode};

use xml::reader::{EventReader, XmlEvent};

use rust_search_engine::{idf, lexer::Lexer, tf, TermFreq, TermFreqIndex};

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

fn index() -> TermFreqIndex {
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
            tf.entry(token).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut stats = tf.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();

        tf_index.insert(file_path, tf);
    }
    tf_index
}

fn run(query: &str) -> Result<(), Box<dyn Error>> {
    let query = &query.chars().collect::<Vec<_>>();
    let tf_index = index();

    let mut result = Vec::<(&Path, f32)>::new();
    for (path, term_freq) in tf_index.iter() {
        println!(
            "{path:?} has {count} unique indexes",
            count = term_freq.len()
        );

        let mut rank = 0f32;
        for token in Lexer::new(query) {
            rank += tf(&token, &term_freq) * idf(&token, &tf_index);
        }

        result.push((&path, rank));
    }
    result.sort_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap());
    result.reverse();

    println!("{result:?}");

    Ok(())
}

fn main() -> ExitCode {
    match run("name, to shader active program") {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
