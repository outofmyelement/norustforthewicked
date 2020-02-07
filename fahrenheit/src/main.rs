use std::io;
//use std::str::FromStr;

fn main() {	
	println!("Please Input a celcius value to convert to fahrenheit");
	let mut guess = String::new();
	io::stdin().read_line(&mut guess)
		.expect("Failed to read line");		
	let value: f32 = guess.trim().parse()
		.expect("Please type a number!");
	println!("{} degrees in celcius is {} degrees in fahrenheit", value, calculate(value));
}

fn calculate(_x: f32) -> f32 {
	let _y = (_x * 1.8) + 32.0;
	_y
}
