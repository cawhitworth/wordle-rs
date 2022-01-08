use std::cmp::Ordering;
use std::collections::{HashMap,HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Position
{
    Any,
    Fixed(usize)
}

struct Constraint
{
    c: char,
    p: Position,
}

fn check_constraint(s1: &str, constraints: &Vec<Constraint>) -> bool
{
    let mut s_mut : String = String::from(s1);
    let mut result = true;
    for constr in constraints {
        result &= match constr.p {
            Position::Fixed(i) => {
                let r = s_mut.chars().nth(i).unwrap() == constr.c;
                s_mut.replace_range(i..i+1, ".");
                r
            },
            Position::Any => {
                match s_mut.chars().position(|c| constr.c == c) {
                    None => false,
                    Some(i) => { s_mut.replace_range(i..i+1, "."); true }
                }
            }
        }
    }
    result
}

fn score_string(s1: &str, freqs: &HashMap<char, usize>) -> usize
{
    s1.chars().map(|c| freqs.get(&c).unwrap()).sum()
}

fn compare_strings(s1: &str, s2: &str, freqs: &HashMap<char, usize>) -> Ordering
{
    score_string(s1, freqs).cmp(&score_string(s2, freqs))
}

fn all_diff(s: &str) -> bool {
    let hs : HashSet<char> = HashSet::from_iter(s.chars());
    hs.len() == s.len()
}

fn main() {
    let c = vec! [
        Constraint { c: 'a', p: Position::Fixed(0) },
        Constraint { c: 's', p: Position::Any }
    ];

    let reader = BufReader::new(File::open("words").expect("Cannot open words"));

    let five_characters: Vec<String> = 
        reader.lines()
              .filter(|w| w.as_ref().unwrap().len() == 5)
              .map(|w| w.unwrap().to_ascii_lowercase())
              .filter(|w| w.chars().all(|c| c.is_ascii_alphabetic()))
              .filter(|w| all_diff(w))
              .collect();

    let mut letter_frequencies: HashMap<char, usize> = HashMap::new();

    for w in &five_characters {
        w.chars().for_each(|c| *(letter_frequencies.entry(c).or_insert(0)) += 1); 
    }

    let mut letter_frequencies_vec : Vec<_> = letter_frequencies.iter().collect();

    letter_frequencies_vec.sort_by(|a,b| b.1.cmp(a.1));

    letter_frequencies_vec.iter().for_each(|(k,v)| println!("{} : {}", k, v) );

    let mut five_characters_vec : Vec<&String> = five_characters
              .iter()
              .filter(|w| check_constraint(w, &c))
              .collect();

    five_characters_vec.sort_by(|s1, s2| compare_strings(s1, s2, &letter_frequencies));

    five_characters_vec.iter().for_each(|f| println!("{} : {}", f, score_string(f, &letter_frequencies)));
}
