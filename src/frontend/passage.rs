use rand::seq::SliceRandom;

pub fn show_passage(count: usize) -> String{
    
    let json = include_str!("../../data/english.json");
    // its the json reader that i js added
    let data: serde_json::Value = 
        serde_json::from_str(json).expect("Wrong json");

    let mut words: Vec<&str> = data["words"]
        .as_array()
        .expect("word array missing")
        .iter()
        .map(|word| word.as_str().expect("word isnt a text ig"))
        .collect();

    words.shuffle(&mut rand::rng());
    words.truncate(count);

    // so we will divide the words into 3 lines
    // because umm monkeytype does it this way

    let words_per_line = words.len().div_ceil(3);

    words.chunks(words_per_line)
        .map(|line| line.join(" "))
        .collect::<Vec<_>>()
        .join("\n")
}