
fn takes_array(array: [u8; 5]) {
    let [first, .., last] = array;
    println!("first: {}, last: {}", first, last);
}

fn takes_tuple(tuple: (char, i32, bool, u8)) {
    let (first, .., last) = tuple;
    println!("first: {}, last: {}", first, last);
}

fn takes_another_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    // This does the same thing as above.
    let (a, b, c) = tuple;

    // Ignore the first element, only bind the second and third.
    let (_, b, c) = tuple;

    // Ignore everything but the last element.
    let (.., c) = tuple;
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

enum Res {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Res {
    if n % 2 == 0 {
        Res::Ok(n / 2)
    } else {
        Res::Err(format!("Cannot divide {n} by 2"))
    }
}

fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    // check optionality
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };
    
    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}

fn main() {
    takes_array([1, 2, 3, 4, 5]);
    takes_tuple(('a', 1, true, 2));
    takes_another_tuple(('a', 1, true));

    let mut foo = Foo { x: (1, 2), y: 3 };
    match &mut foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => {
            println!("y = 2, x = {i:?}");
            i.0 = 3;
        },
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }

    let n = 101;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}
