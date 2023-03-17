use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 15);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 15];

    let scores2:HashMap<_ ,_> = teams.iter().zip(initial_scores.iter()).collect();

    let text = "hello world wonderful word";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }

    println!("{:?}",map);
}
