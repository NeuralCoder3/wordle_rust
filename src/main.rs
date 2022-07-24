use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, stdout, BufRead, Write};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let mut trie = Trie::new();

    let filename = "5_simpl.txt";
    for line in read_lines(filename).unwrap() {
        let line = line.unwrap();
        let words = line.split_whitespace();
        for word in words {
            trie.insert(word);
            // println!("inserted {}", word);
        }
    }

    let word_list = trie.list_content("");
    for word in trie.list_content("") {
        println!("{}", word);
    }

    let chosen_word = word_list
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    println!("The chosen word is \"{}\"", chosen_word);

    let mut round = 0;
    let mut won = false;
    while !won {
        round += 1;
        // askes for input until a valid guess (in trie) is entered
        let mut guess = String::new();
        loop {
            print!("Enter your guess: ");
            stdout().flush().unwrap();
            guess.clear();
            io::stdin().read_line(&mut guess).unwrap();
            guess = guess.trim().to_string();
            if trie.contains(&guess) {
                break;
            } else {
                println!("{} is not in the trie.", guess);
            }
        }
        // checks if the guess is the chosen word
        if guess == chosen_word {
            println!("You won!");
            won = true;
        } else {
            // println!("You guessed {}", guess);
            let feedback = get_feedback(&chosen_word, &guess);
            println!("{}", feedback);
        }
    }
    println!("You won in {} rounds!", round);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_histogram(word: &str) -> Vec<u8> {
    let mut histogram = vec![0; 26];
    for c in word.chars() {
        let index = c as usize - 'a' as usize;
        histogram[index] += 1;
    }
    histogram
}

fn get_feedback(chosen_word: &str, guess: &str) -> String {
    let mut feedback = String::new();
    let mut histogram = get_histogram(chosen_word);
    for (i, c) in guess.chars().enumerate() {
        let index = c as usize - 'a' as usize;
        if c == chosen_word.chars().nth(i).unwrap() {
            feedback.push('🟩');
            histogram[index] -= 1;
        } else if chosen_word.contains(c) && histogram[index] > 0 {
            feedback.push('🟨');
            histogram[index] -= 1;
        } else {
            feedback.push('⬛');
        }
    }
    feedback
}




#[derive(Clone)]
struct Trie {
    children: Vec<Trie>,
    is_word: bool,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            children: Vec::new(),
            is_word: false,
        }
    }

    fn insert(&mut self, word: &str) {
        if word.len() == 0 {
            self.is_word = true;
            return;
        }

        let index = word.chars().next().unwrap() as usize - 'a' as usize;
        if self.children.len() <= index {
            self.children.resize(index + 1, Trie::new());
        }

        let child = &mut self.children[index];
        child.insert(&word[1..]);
    }

    fn contains(&self, word: &str) -> bool {
        if word.len() == 0 {
            return self.is_word;
        }

        let index = word.chars().next().unwrap() as usize - 'a' as usize;
        if self.children.len() <= index {
            return false;
        }

        let child = &self.children[index];
        child.contains(&word[1..])
    }

    fn list_content(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        if self.is_word {
            result.push(prefix.to_string());
        }

        for (i, child) in self.children.iter().enumerate() {
            let mut child_prefix = prefix.to_string();
            child_prefix.push((i as u8 + 'a' as u8) as char);
            result.extend(child.list_content(&child_prefix));
        }

        result
    }
}
