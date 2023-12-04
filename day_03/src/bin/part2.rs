fn main() {
	println!("(part 2)");
	let input: &str    = include_str!("./input2.txt");
	let output: String = part2( input);
	dbg!(output);
}

fn part2(input: &str) ->String {
	"todo!()".to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_2() {
		let input: &str    = include_str!("./test2.txt");
		let result: String = part2(input);
		assert_eq!(result, "todo!()".to_string());
	}
}