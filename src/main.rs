use std::env;

const NOTES: [&str; 12] = ["Do", "Do#", "Ré", "Ré#", "Mi", "Fa", "Fa#", "Sol", "Sol#", "La", "La#", "Si"];
const MAJOR: [usize; 7] = [2,2,1,2,2,2,1];
const MINOR: [usize; 7] = [2,1,2,2,1,2,2];
const DORIEN: [usize; 7] = [2, 1, 2, 2, 2, 1, 2];
const PHRYGIEN: [usize; 7] = [1, 2, 2, 2, 1, 2, 2];
const LYDIEN: [usize; 7] = [2, 2, 2, 1, 2, 2, 1];
const MIXOLYDIEN: [usize; 7] = [2, 2, 1, 2, 2, 1, 2];
const EOLIEN: [usize; 7] = [2, 1, 2, 2, 1, 2, 2];
const LOCRIEN: [usize; 7] = [1, 2, 2, 1, 2, 2 , 2];

fn analyse_args(args: &Vec<String>) -> (&str, &[usize]) {
    let note = &args[1];
    let mode = match &args[2].as_str() {
        &"major" => &MAJOR,
        &"minor" => &MINOR,
        &"ionien" => &MAJOR,
        &"dorien" => &DORIEN,
        &"phrygien" => &PHRYGIEN,
        &"lydien" => &LYDIEN,
        &"mixolydien" => &MIXOLYDIEN,
        &"eolien" => &EOLIEN,
        &"locrien" => &LOCRIEN,
        &_ => &MAJOR
    };

    (note, mode)
}

// Calcul les notes d'une gamme en fonction d'un mode
fn calculate_gamme<'a>(basic_note: &str, gamme_mod: &[usize]) -> Vec<&'a str> {
    let index = NOTES.iter().position(|&s| s == basic_note).unwrap();
    let intervals = gamme_mod;
    let mut gamme: Vec<&str> = Vec::new();
    let mut idx = index % 12;
    for ton in intervals.iter() {
        idx = idx % 12;
        gamme.push(NOTES[idx]);
        idx = idx + ton;
    }
    
    gamme
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("arguments {:?}", args);
    let (note, mode) = analyse_args(&args);
    let gamme: Vec<&str> = calculate_gamme(note, mode);
    println!("{:?}", gamme);
}
 