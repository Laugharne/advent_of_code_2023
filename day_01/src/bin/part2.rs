fn main() {
	println!("(part 2)");
	let input: &str    = include_str!("./input1.txt");
	let output: String = part2( input);
	println!();
	dbg!(output);
}

fn part2(input: &str) ->String {
	let lines : Vec<&str>     = input.lines().collect();
	let mut sum: u32          = 0;
	let digit_word: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

	lines.iter().filter(|&&line| !line.trim().is_empty()).for_each(|line| {
		//println!("{:?}", *line);
		let mut digit_1st: u32 = 0;
		let mut pos_1st: u32   = u32::MAX;
		if let Some((position, digit_char)) = line.chars().enumerate().find(|&(_, c)| c.is_ascii_digit()) {
			let digit_value = digit_char.to_digit(10).expect("Erreur lors de la conversion en chiffre");
			
			pos_1st   = position as u32;
			digit_1st = digit_value;
		}

		let mut digit_w_1st: u32 = 0;
		let mut pos_w_1st: u32   = u32::MAX;

		for (idx, word) in (0_u32..).zip(digit_word.into_iter()) {
			if let Some(position) = line.find(word) {
				//println!("{} Trouvé à la position : {}", word, position);
				if position < (pos_w_1st as usize) {
					pos_w_1st   = position as u32;
					digit_w_1st = idx + 1;
				}
			}
		}

		let digit_1st: u32 = match pos_1st.cmp(&pos_w_1st) {
			std::cmp::Ordering::Less => {
				digit_1st
			}
			_ => digit_w_1st
		};



		let mut digit_last: u32 = 0;
		let mut pos_last: u32   = u32::MAX;
		if let Some((position, digit_char)) = line.chars().rev().enumerate().find(|&(_, c)| c.is_ascii_digit()) {
			let digit_value = digit_char.to_digit(10).expect("Erreur lors de la conversion en chiffre");
			
			pos_last   = position as u32;
			digit_last = digit_value;
		}


		let enil: String = line.chars().rev().collect();

		let digit_drow: [&str; 9] = ["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
		let mut digit_w_last: u32 = 0;
		let mut pos_w_last: u32   = u32::MAX;

		for (idx, word) in (0_u32..).zip(digit_drow.into_iter()) {
			if let Some(position) = enil.find(word) {
				//println!("{} Trouvé à la position : {}", word, position);
				if position < (pos_w_last as usize) {
					pos_w_last   = position as u32;
					digit_w_last = idx + 1;
				}
			}
		}

		let digit_last: u32 = match pos_last.cmp(&pos_w_last) {
			std::cmp::Ordering::Less => {
				digit_last
			}
			_ => digit_w_last
		};

		sum += digit_1st*10 + digit_last;
	});

	sum.to_string()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_2() {
		let input: &str    = include_str!("./test2.txt");
		let result: String = part2(input);
		assert_eq!(result, "281".to_string());
	}
}