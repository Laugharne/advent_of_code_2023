fn main() {
	println!("(part 2)");
	let input: &str    = include_str!("./input2.txt");
	let output: String = part2( input);
	dbg!(output);
}


fn split_string( string: &str, delimiter: char) -> Vec<&str> {
	let splited: Vec<&str> = string.split_terminator(delimiter).collect();
	splited
}


fn part2(input: &str) ->String {
	let lines: Vec<&str>  = input.lines().collect();
	let mut sum: i32      = 0;
	
	lines.iter().filter(|&&line| !line.trim().is_empty()).for_each(|line| {
		//println!("{:?}", *line);
		let game_id_list_subsets_cubes: Vec<&str> = split_string(line, ':');
		let game_id: Vec<&str> = split_string(game_id_list_subsets_cubes[0], ' ');
		let id: i32 = game_id[1].parse::<i32>().unwrap();
		println!("ID {}", id);

		let mut counter_red: i32   = 0;
		let mut counter_green: i32 = 0;
		let mut counter_blue: i32  = 0;

		let subsets_cubes: Vec<&str> = split_string(game_id_list_subsets_cubes[1], ';');
		subsets_cubes.iter().enumerate().for_each(|subset| {
			println!("  {}", subset.1);
			let cubes: Vec<&str> = split_string(subset.1, ',');
			cubes.iter().enumerate().for_each(|cube| {
				let qty_color: Vec<&str> = split_string(cube.1.trim(), ' ');
				let qty: i32 = qty_color[0].parse().unwrap();
				match qty_color[1].trim() {
					"red"   => { counter_red   = std::cmp::max(counter_red,   qty);},
					"green" => { counter_green = std::cmp::max(counter_green, qty);},
					"blue"  => { counter_blue  = std::cmp::max(counter_blue,  qty);},
					_       => {},
				}

			});// cubes: "3 blue, 4 red"

		});// subsets : "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"

		let power = counter_red * counter_green * counter_blue;
		println!("\n   r:{}\tg:{}\tb:{}\t{}", counter_red, counter_green, counter_blue, power);
		sum += power;
		

		println!();
	}); // line : "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"

	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_2() {
		let input: &str    = include_str!("./test2.txt");
		let result: String = part2(input);
		assert_eq!(result, "2286".to_string());
	}
}