
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
}
