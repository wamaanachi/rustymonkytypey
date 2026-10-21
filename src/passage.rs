pub const WORDS: &[&str] = &["umm", "sure", "ig", "lowk", "idk", "what",
               "to", "put", "here", "just", "a", "lot", "of",
               "random", "words", "lol", "blue", "is", "really",
               "cool", "i", "am", "gonna", "stop", "now",
               "i", "want", "it", "that", "way", "tell","me",
               "why", "aint", "nothing", "but", "a", "heartache",
               "mistake", "never", "hear", "say", "hackatime", "is", "bad",
               "bruhh", "working", "or", "nah", "acting", "weird"];

// we need like 43 - 45 words 
// but ill add more words in the future

use rand::seq::SliceRandom;

pub fn show_passage(){
    let mut words = WORDS.to_vec();
    words.shuffle(&mut rand::rng());

    // so we will divide the words into 3 lines
    // because umm monkeytype does it this way

    let words_per_line = words.len().div_ceil(3);

    for line in words.chunks(words_per_line){
        println!("{}", line.join(" "));
    }
}