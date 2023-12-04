fn main() {
	println!("(part 1)");
	let input: &str    = include_str!("./input1.txt");
	let output: String = part1( input);
	println!();
	dbg!(output);
}

fn part1(input: &str) ->String {
	let lines : Vec<&str> = input.lines().collect();

	lines.iter().filter(|&&line| !line.trim().is_empty()).for_each(|line| {
		println!("{:?}", *line);

		// TODO
	});

	"todo!()".to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_1() {
		let input: &str    = include_str!("./test1.txt");
		let result: String = part1(input);
		assert_eq!(result, "4361".to_string());
	}
}