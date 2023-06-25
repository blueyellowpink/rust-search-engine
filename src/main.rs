use std::{fs, io, path::Path};

use xml::reader::{EventReader, XmlEvent};

use rust_search_engine::{calculate_tf, lexer::Lexer, TermFreq, TermFreqIndex};

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
            tf.entry(token).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut stats = tf.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();

        tf_index.insert(file_path, tf);
    }

    let query = &"name shader".chars().collect::<Vec<_>>();
    let mut result = Vec::<(&Path, f32)>::new();
    for (path, tf) in tf_index.iter() {
        println!("{path:?} has {count} unique indexes", count = tf.len());

        let mut total_tf = 0f32;
        for token in Lexer::new(query) {
            total_tf += calculate_tf(&token, &tf);
        }

        result.push((&path, total_tf));
    }
    result.sort_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap());
    result.reverse();

    println!("{result:?}");
}
