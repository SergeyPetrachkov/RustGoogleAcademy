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
		println!("{i}");
		if i > 100 {
			break;
		}
	}
}

fn main() {
	blocks();
	if_else();
	if_as_expression();
	match_expressions();
	match_returning_values();
	while_loop();
	for_loop();
	loop_loop();
}