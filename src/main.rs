use std::io;

fn main() {
	
	let mut fahrenheit = String::new();

	println!("Converter fahrenheit to celsius 0.1.0");
	println!("Please input fahrenheit: ");

	io::stdin().read_line(&mut fahrenheit)
		.expect("Faild to read line");

	let fahrenheit: i32 = fahrenheit.trim().parse()
		.expect("Please type a number!");

	let celsius: i32 = {
		(fahrenheit - 32) * 5/9
	};
	
	println!("celsius is {}", celsius);
}
