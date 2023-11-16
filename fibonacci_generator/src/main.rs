use std::io;

fn main() {
	loop {
		let mut num = String::new();
		println!("Enter number:");
		let _ = io::stdin().read_line(&mut num);
		let num = num.trim().parse::<u8>().expect("invalid number");
		println!("Number in fibonacci sequence: {:#}", fibonacci(num).to_string())
	}
}

fn fibonacci (n: u8) -> u128{
	let index = if n > 186 {186} else {n};
	let mut last_value:[u128; 256] = [1; 256];
	for i in 2..index {
		last_value[i as usize] = last_value[(i-2) as usize] + last_value[(i-1) as usize];
	}
	last_value[(index-1) as usize]
}
