use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::cmp::Ordering;

fn main() {

    // create a list of possible words: words from the list of 'all_words.txt' that have a length <= 6
    let mut input_path = Path::new("all_words.txt");
    let f: File = File::open(&input_path).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut fo = File::create("scrabble_words_with_l6.txt").expect("Unable to create file");

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let trimmed_line = line.trim();

        if trimmed_line.len() <= 6 {
            fo.write_all(trimmed_line.as_bytes()).expect("Unable to write data");
            fo.write_all("\n".as_bytes()).expect("Unable to write data");
        }
    }

    // determine playable words
    // 1. for each word in the complete list of 6-character words
    // 2. determine if each letter in word is in list of playable letters
    // 3. delete the letter from playable letters and move to next letter
    // 4. if the letter isnt there, go on to next word
    // let letters= vec!['a', 'b', 'c', 'd', 'e', 'f']; // playable letters

    input_path = Path::new("scrabble_words_with_l6.txt");
    let f: File = File::open(&input_path).expect("Unable to open file");
    let f: BufReader<File> = BufReader::new(f);

    let mut fo = File::create("playable_words.txt").expect("Unable to create file");

    let mut playable_words: Vec<String> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        let mut letters= vec!['E', 'L', 'A', 'N', 'M', 'T']; // playable letters; initiate here so that it resets after potential deletes.

        // our word to check
        let word = line.trim().to_string();
        let mut is_valid: bool = true;

        // checking that each letter is available
        for letter in word.chars() {
            if letters.contains(&letter) {
                // remove the letter from playable characters
                let index = letters.iter().position(|x| *x == letter).unwrap();
                letters.remove(index);
                if letters.is_empty() { break; }
            } else {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            // append 'word' to 'playable_words'
            playable_words.push(word);
        }
        
    }

    // sort the playable words so that larger words appear first in the file
    playable_words.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        } else if a.len() == b.len() {
            Ordering::Equal
        } else { Ordering::Greater }
    });

    // write all words to file for easier viewing
    for word in playable_words {
        fo.write_all(word.as_bytes()).expect("Unable to write data");
        fo.write_all("\n".as_bytes()).expect("Unable to write data");
    }

}
