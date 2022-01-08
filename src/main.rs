use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn compare_strings(s1: &str, s2: &str, freqs: &HashMap<char, u32>) -> Ordering
{
    let score1 : u32 = s1.chars().map(|c| freqs.get(&c).unwrap()).sum();
    let score2 : u32 = s2.chars().map(|c| freqs.get(&c).unwrap()).sum();
    
    score1.cmp(&score2)
}

fn main() {
    let reader = BufReader::new(File::open("words").expect("Cannot open words"));

    let five_characters: Vec<String> = 
        reader.lines()
              .filter(|w| w.as_ref().unwrap().len() == 5)
              .map(|w| w.unwrap().to_ascii_lowercase())
              .filter(|w| w.chars().all(|c| c.is_ascii_alphabetic()))
              .collect();

    let mut letter_frequencies: HashMap<char, u32> = HashMap::new();

    for w in &five_characters {
        w.chars().for_each(|c| *(letter_frequencies.entry(c).or_insert(0)) += 1); 
    }

    let mut letter_frequencies_vec : Vec<_> = letter_frequencies.iter().collect();

    letter_frequencies_vec.sort_by(|a,b| b.1.cmp(a.1));

    for (k,v) in letter_frequencies_vec {
        println!("{} : {}", k, v);
    }

    let mut five_characters_vec : Vec<_> = five_characters.clone();
    five_characters_vec.sort_by(|s1, s2| compare_strings(s1, s2, &letter_frequencies));

    five_characters_vec.iter().for_each(|f| println!("{}", f));
}
