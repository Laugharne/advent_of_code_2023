fn main() {
	println!("(part 1)");
	let input: &str    = include_str!("./input1.txt");
	let output: String = part1( input);
	println!();
	dbg!(output);
}


fn split_string( string: &str, delimiter: char) -> Vec<&str> {
	let splited: Vec<&str> = string.split_terminator(delimiter).collect();
	splited
}


fn part1(input: &str) ->String {
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

		println!("\n   r:{} g:{} b:{}", counter_red, counter_green, counter_blue);
		if (counter_red <= 12) && (counter_green <= 13) && (counter_blue <= 14) {
			sum += id;
			println!("+");
		} else {
			println!(" ");
		}

		println!();
	}); // line : "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"

	sum.to_string()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_1() {
		let input: &str    = include_str!("./test1.txt");
		let result: String = part1(input);
		assert_eq!(result, "8".to_string());
	}

	#[test]
	fn split_colon_1() {
		let result: Vec<&str> = split_string(
			"Game 1: 3 blue, 4 red",
			':'
		);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0], "Game 1");
		assert_eq!(result[1], " 3 blue, 4 red");
	}

	#[test]
	fn split_colon_2() {
		let result: Vec<&str> = split_string(
			"Game 100: 8 red, 2 blue, 1 green; 2 blue, 4 red, 2 green; 9 red, 1 green; 2 green, 2 red; 3 red, 5 blue; 5 blue, 8 red",
			':'
		);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0], "Game 100");
		assert_eq!(result[1], " 8 red, 2 blue, 1 green; 2 blue, 4 red, 2 green; 9 red, 1 green; 2 green, 2 red; 3 red, 5 blue; 5 blue, 8 red");
	}

	#[test]
	fn split_game_id_1() {
		let result: Vec<&str> = split_string(
			"Game 1",
			' '
		);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0], "Game");
		assert_eq!(result[1], "1");
	}

	#[test]
	fn split_game_id_100() {
		let result: Vec<&str> = split_string(
			"Game 100",
			' '
		);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0], "Game");
		assert_eq!(result[1], "100");
	}

	#[test]
	fn split_xyz_id() {
		let result: Vec<&str> = split_string(
			"XYZ 100",
			' '
		);
		assert_eq!(result.len(), 2);
		assert_ne!(result[0], "Game");
		assert_eq!(result[1], "100");
	}

	#[test]
	fn split_semicolon_1() {
		let result: Vec<&str> = split_string(
			" 6 red, 1 blue, 3 green",
			';'
		);
		assert_eq!(result.len(), 1);
		assert_eq!(result[0], " 6 red, 1 blue, 3 green");
	}

	#[test]
	fn split_semicolon_2() {
		let result: Vec<&str> = split_string(
			" 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
			';'
		);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0], " 6 red, 1 blue, 3 green");
		assert_eq!(result[1], " 2 blue, 1 red, 2 green");
	}

	#[test]
	fn split_semicolon_3() {
		let result: Vec<&str> = split_string(
			" 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			';'
		);
		assert_eq!(result.len(), 3);
		assert_eq!(result[0], " 3 blue, 4 red");
		assert_eq!(result[1], " 1 red, 2 green, 6 blue");
		assert_eq!(result[2], " 2 green");
	}

	#[test]
	fn split_comma_1() {
		let result: Vec<&str> = split_string(
			" 7 green",
			','
		);
		assert_eq!(result.len(), 1);
		assert_eq!(result[0], " 7 green");
	}

	#[test]
	fn split_comma_2() {
		let result: Vec<&str> = split_string(
			" 3 blue, 4 red",
			','
		);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0], " 3 blue");
		assert_eq!(result[1], " 4 red");
	}

	#[test]
	fn split_comma_3() {
		let result: Vec<&str> = split_string(
			" 1 red, 2 green, 6 blue",
			','
		);
		assert_eq!(result.len(), 3);
		assert_eq!(result[0], " 1 red");
		assert_eq!(result[1], " 2 green");
		assert_eq!(result[2], " 6 blue");
	}

}
