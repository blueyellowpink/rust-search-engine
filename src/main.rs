use std::{
    error::Error,
    fs,
    io::{self, BufReader},
    path::Path,
    process::ExitCode,
};

use xml::reader::{EventReader, XmlEvent};

use rust_search_engine::{
    compute_idf, compute_tf, lexer::Lexer, DocFreq, SearchEngine, TermFreq, TermFreqIndex,
};

fn read_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = fs::File::open(file_path)?;
    let file = BufReader::new(file);
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

fn index() -> SearchEngine {
    let dir_path = "docs.gl/test";
    let dirs = fs::read_dir(dir_path).unwrap();

    let mut doc_freq = DocFreq::new();
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

        let mut total_freq = 0;
        for (term, freq) in &tf {
            doc_freq
                .entry(term.to_string())
                .and_modify(|x| *x += 1)
                .or_insert(1);
            total_freq += freq;
        }

        tf_index.insert(file_path, (tf, total_freq));
    }

    SearchEngine {
        df: doc_freq,
        index: tf_index,
    }
}

fn rank(engine: &SearchEngine, query: &str) -> Result<(), Box<dyn Error>> {
    let query = &query.chars().collect::<Vec<_>>();
    let mut result = Vec::<(&Path, f32)>::new();
    for (path, term_freq) in &engine.index {
        println!(
            "{path:?} has {count} unique indexes",
            count = term_freq.0.len()
        );

        let mut rank = 0f32;
        for token in Lexer::new(query) {
            rank += compute_tf(&token, &term_freq.0, term_freq.1)
                * compute_idf(&token, &engine.df, engine.index.len());
        }

        result.push((&path, rank));
    }
    result.sort_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap());
    result.reverse();

    println!("{result:?}");

    Ok(())
}

fn main() -> ExitCode {
    let engine = index();

    match rank(&engine, "name, to shader active program") {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
