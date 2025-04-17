use std::any::type_name_of_val;

fn blocks() {
	let z = 13;
	let x = {
		let y = 10;
		println!("y: {y}");
		z - y
	};
	println!("x: {x}");
	// println!("y: {y}");
}

fn if_else() {
	let x = 109;
	if x == 0 {
		println!("zero!");
	} else if x < 100 {
		println!("biggish");
	} else {
		println!("huge");
	}
}

fn if_as_expression() {
	let x = 10;
	let size = if x < 20 { "small" } else { "large" };
	println!("number size: {}", size);
	println!("type of size: {}", type_name_of_val(size))
}

fn match_expressions() {
	let val = 102;
		match val {
		1 => println!("one"),
		10 => println!("ten"),
		100 => { 
			println!("one hundred");
			println!("one hundred indeed");
		},
		_ => { // like default?
			println!("something else");
			println!("val: {val}");
		}
	}
}

fn match_returning_values() {
	let flag = false;
	let val = match flag {
		true => 1,
		false => 0,
	};
	println!("The value of {flag} is {val}");
}

fn while_loop() {
	let mut x = 200;
	while x >= 10 {
		x = x / 2;
	}
	println!("Final x: {x}");
}

fn for_loop() {
	for x in 1..5 {
		println!("x: {x}");
	}

	for x in 1..=5 {
		println!("x: {x}");
	}

	for elem in [2, 4, 8, 16, 32] {
		println!("elem: {elem}");
	}
}

fn loop_loop() {
	let mut i = 0;
	loop {
		i += 1;
		dbg!(i);
		if i > 100 {
			break;
		}
	}
}

fn fizzbuzz(n: u32) -> u32 {
	return 1;
	todo!("Demonstrate macros! n = {n}")
}

fn collatz_sequence(mut n: i32) -> u32 {
	let mut sequence_length = 1;

	while n > 1 {
		n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
		sequence_length += 1;
	}

	sequence_length
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
	let mut transposed = [[0; 3]; 3];

	for i in 0..matrix.len() {
		for j in 0..matrix[i].len() {
			transposed[j][i] = matrix[i][j];
		}
	}

	transposed
}


fn main() {
	// blocks();
	// if_else();
	// if_as_expression();
	// match_expressions();
	// match_returning_values();
	// while_loop();
	// for_loop();
	// loop_loop();
	// fizzbuzz(1);
	// dbg!(collatz_sequence(11));

	let matrix = [
		[101, 102, 103],
		[201, 202, 203],
		[301, 302, 303],
	];

	dbg!(matrix);
	let transposed = transpose(matrix);
	dbg!(transposed);
}