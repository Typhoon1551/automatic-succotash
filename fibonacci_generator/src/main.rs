use std::io::stdin;
 
fn main() {
	loop {
		let mut num = String::new();
		println!("Enter number:");

		let _ = stdin().read_line(&mut num);

		if num.trim() == "quit" {
			println!("Quitting");
			break;
		}

		let num = num.trim().parse::<u8>().expect("invalid number");

		if num > 186 {
			println!("Number too large, displaying output for 186 instead");
		}

		println!("Number in fibonacci sequence: {:#}", fibonacci(&num))
	}
}

fn fibonacci (n: &u8) -> u128{
	let index = if n > &186 {186} else {n.to_owned()};
	let mut last_value:[u128; 256] = [1; 256];
	for i in 2..index {
		last_value[i as usize] = last_value[(i-2) as usize] + last_value[(i-1) as usize];
	}
	last_value[(index-1) as usize]
}
