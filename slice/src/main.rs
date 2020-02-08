use std::io;

fn main() {
	//Write a function that takes a string and returns the first word it finds in that string.
	//If the function doesn't find a space, the whole string must be on word: return the whole string
	let mut sentence = String::new();
	io::stdin().read_line(&mut sentence)
		.expect("Failed to read line");		

println!("{}",	first_word(&sentence));
}

fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0...i];
		}
	}
	&s[..]
}

