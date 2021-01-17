// todo: better handling of punctuations.

use rand::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_stories() -> String {
    let mut text = String::with_capacity(4_000_000);
    for line in BufReader::new(File::open("assets/cano.txt").unwrap())
        .lines()
        .skip(156)
    {
        if let Ok(line) = line {
            let line = line.trim().to_lowercase();
            if line.is_empty() || (line.len() < 50 && line.as_bytes()[line.len() - 1] != b'.') {
                continue;
            }
            text.push_str(&line);
            text.push(' ');
        }
    }
    text
}

fn clean_text(text: String) -> String {
    let p = Regex::new(r"[^\w\s]").unwrap();
    p.replace_all(&text, "").into_owned()
}

type MarkovModel = HashMap<String, Vec<(String, f64)>>;

fn build_markov_model(words: Vec<&str>, n: usize) -> MarkovModel {
    let mut model: HashMap<String, HashMap<String, f64>> = HashMap::with_capacity(100_000);
    let n_grams = words.windows(n).collect::<Vec<_>>();
    for i in 0..n_grams.len() - 1 - n {
        let curr_state = n_grams[i].join(" ");
        let next_state = n_grams[i + n].join(" ");
        let curr_state = model.entry(curr_state).or_default();
        let count = curr_state.entry(next_state).or_default();
        *count += 1.0;
    }
    for next_states in model.values_mut() {
        let sum: f64 = next_states.values().sum();
        for v in next_states.values_mut() {
            *v /= sum;
        }
    }
    model
        .into_iter()
        .map(|(curr_state, next_state)| (curr_state, next_state.into_iter().collect()))
        .collect()
}

fn generate_story(model: &MarkovModel, limit: usize, start: &str) -> String {
    let mut rng = thread_rng();
    let n = start.split_whitespace().count();
    let mut story = String::with_capacity(limit * n);
    story.push_str(start);
    story.push(' ');
    let mut curr = start;
    for _ in 0..limit / n {
        if let Some(next_states) = model.get(curr) {
            let next = &next_states.choose_weighted(&mut rng, |x| x.1).unwrap().0;
            story.push_str(next);
            curr = &next;
        } else {
            curr = start;
            story.push('.');
        }
        story.push(' ');
    }
    story
}

fn main() {
    let stories = clean_text(read_stories());
    let words = stories.split(' ').collect::<Vec<_>>();
    let model = build_markov_model(words, 2);
    let story = generate_story(&model, 200, "he was");
    println!("{:?}", story);
}
