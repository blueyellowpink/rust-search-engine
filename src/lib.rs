use std::{collections::HashMap, path::PathBuf};

pub mod lexer;

pub type TermFreq = HashMap<String, usize>;
pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;

pub fn tf(t: &str, d: &TermFreq) -> f32 {
    let a = d.get(t).cloned().unwrap_or(0) as f32;
    let b = d.iter().map(|(_, f)| *f).sum::<usize>() as f32;
    a / b
}

pub fn idf(t: &str, d: &TermFreqIndex) -> f32 {
    let total_doc = d.len() as f32;
    let count = d.values().filter(|tf| tf.contains_key(t)).count().max(1) as f32;
    (total_doc / count).log10()
}
