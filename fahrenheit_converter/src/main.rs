use std::io;

fn main() {
	println!("Insert a Celcius value to convert to Fahrenheit: ");
	let mut cel = String::new();
	io::stdin().read_line(&mut cel).expect("Failed to read line");
	let _f = cel.parse::<f32>();
	println!("{}", _f);
//	println!("{} in Fahrenheit is {}", _f, calculation(_f));
//	println!("{} in Fahrenheit is {}", cel, calculation(cel));
}

fn calculation(x: f32) -> f32 {
	let _fahr = (x * 1.8).floor();
	5.0
}
