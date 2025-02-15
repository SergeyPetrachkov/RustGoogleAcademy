fn fibonacci(n: u32) -> u32 {
	if n < 2 {
		return n;
	} else {
		return fibonacci(n - 1) + fibonacci(n - 2);
	}
}

fn main() {
	let x = fibonacci(10);
	println!("x = {x}");
}