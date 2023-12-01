fn main() {
	println!("(part 1)");
	let input: &str    = include_str!("./input1.txt");
	let output: String = part1( input);
	println!();
	dbg!(output);
}

fn part1(input: &str) ->String {
	let lines : Vec<&str> = input.lines().collect();
	let mut sum: u32 = 0;

	lines.iter().filter(|&&line| !line.trim().is_empty()).for_each(|line| {
		println!("{:?}", *line);
		let digit_1st: u32   = line.chars().find(|c| c.is_ascii_digit()).expect("error 1st digit").to_digit(10).unwrap();
		let digit_last: u32  = line.chars().rev().find(|c| c.is_ascii_digit()).expect("error last digit").to_digit(10).unwrap();

		sum += digit_1st*10 + digit_last;
	});

	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_1() {
		let input: &str    = include_str!("./test1.txt");
		let result: String = part1(input);
		assert_eq!(result, "142".to_string());
	}
}