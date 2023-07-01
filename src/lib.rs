use std::{collections::HashMap, path::PathBuf};

pub mod lexer;

pub type DocFreq = HashMap<String, usize>;
pub type TermFreq = HashMap<String, usize>;
pub type TermFreqIndex = HashMap<PathBuf, (TermFreq, usize)>;

pub struct SearchEngine {
    pub index: TermFreqIndex,
    pub df: DocFreq,
}

pub fn compute_tf(t: &str, d: &TermFreq, n: usize) -> f32 {
    let n = n as f32;
    let freq = d.get(t).cloned().unwrap_or(0) as f32;
    freq / n
}

pub fn compute_idf(t: &str, df: &DocFreq, n: usize) -> f32 {
    let n = n as f32;
    let count = df.get(t).cloned().unwrap_or(1) as f32;
    (n / count).log10()
}
