# iOS Dev on a Rust journey #3 Generics

Hello there! This is the 3d record in my journal. Today we're gonna talk about generics.
Generics are an important part of any modern language, but not all languages have the same set of tools.
Swift and Rust are both relatively young languages, and I'll be talking from the perspective of a Swift developer, since this is what I'm relatively good at.

We will compare the features of both languages, look at similarities and differences. 

## Why Generics?

Why do we need them? If we want to write code that can work with different data types, but still be type-safe, generics are what we need.
This goes hand in hand with SOLID, DRY, POP (protocol oriented programming in Swift) and other fancy abbreviations. 
But in short, the benefits of generics are:

* Code reusability
* Type safety
* Better modularity
* Sometimes performance. Depends on the language and it's compiler, but since we're talking about Swift and Rust, it's a fair statement.

## Basic example

Every other article will mention something like swapping two variables. Ok, let's do it.

Swift version:
```Swift
func swapValues<T>(lhs: inout T, rhs: inout T) {
    let temp = lhs
    lhs = rhs
    rhs = temp
}
```

and in Rust:

```Rust
fn swap_values<T>(lhs: &mut T, rhs: &mut T) {
    std::mem::swap(lhs, rhs);
}
```

Well, Rust looks a bit shorter, but that's not the point :) 
The point is that we define a generic function, that accepts two arguments of type T (generic, any type we want) and swaps them.

This basic functionality exists in almost all languages (maybe in all of them? at least in all that I know).

But the great things start with more advanced stuff, like constraining generics. And we need to do a little pause and talk about abstractions.

## Protocols vs Traits

If you want to define a behavior that some type should conform to, you create a protocol in Swift, or a trait in Rust. (I think I first heard the term trait during my short acquaintance with PHP).
1. In Swift, you can define properties and functions and associated types, in Rust you can define functions and associated types.
2. Both traits and protocols support default implementations for the functions. In Swift, you do that in an extension, in Rust - just inside the trait.
3. Both traits and protocols support composition.
4. Dynamic dispatch can be achieved with `any Protocol` in Swift and `dyn Trait` in Rust.
5. Static dispatch can be achieved via generics + constraints in Swift. In Rust, it's default.

In Swift:
```Swift
protocol Person {
   associatedType Job

   var name: String { get set }
   var age: Int { get set }
   var job: Job { get }
   func intro() -> String
}

extension Person {
   func intro() -> String {
       "Hello! I'm \(name), and I'm \(age) years old. I do \(job) for a living."
   }
}
```

In Rust:
```Rust
trait Person {
    type Job;

    fn name(&self) -> &str;
    fn set_name(&mut self, name: String);

    fn age(&self) -> u32;
    fn set_age(&mut self, age: u32);

    fn job(&self) -> &Self::Job;

    fn intro(&self) -> String {
        format!(
            "Hello! I'm {}, and I'm {} years old. I do {} for a living.",
            self.name(),
            self.age(),
            self.job()
        )
    }
}
```

Ok, good. We've scratched the surface of the protocols and traits. But we've also just discussed Associated Types.
So, Person is a generic trait/protocol, it has an associated type Job. And this leads us to the next generic feature.

## Constraining Generics

Let's create a dummy function that will print a description of any type that conforms to a certain protocol/trait.

In Swift:
```Swift
func printDescription<T: CustomStringConvertible>(_ value: T) {
    print(value.description)
}
```

In Rust:
```Rust
fn print_description<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

In both Swift and Rust, the constraints define the limit of the types that can be used. 

In other words, this function can only be called on a type that conforms to `CustomStringConvertible` in Swift, or `Display` in Rust.
If you try to call it on other types, it won't compile.

## Generic Types

Besides functions and protocols/traits, we can define generic types Structs/Enums/Classes in Swift and Structs/Enums in Rust.

Swift:
```Swift
struct Pair<T, U> {
    let first: T
    let second: U
}

enum Either<First, Second> {
    case first(First)
    case second(Second)
}

final class Logger<LogDestination> {
    private let destination: LogDestination
    
    init(destination: LogDestination) {
        self.destination = destination
    }
}
```

Rust:
```Rust
struct Pair<T, U> {
    first: T,
    second: U,
}

enum Either<First, Second> {
    First(First),
    Second(Second),
}

struct Logger<LogDestination> {
    destination: LogDestination,
}

impl<LogDestination> Logger<LogDestination> {
    fn new(destination: LogDestination) -> Self {
        Logger { destination }
    }
}
```

## Extensions vs Trait Implementations

Let's extend our type Pair, so we can get a formatted description of an object.

In Swift:
```Swift
extension Pair: CustomStringConvertible where T: CustomStringConvertible, U: CustomStringConvertible {
    var description: String {
        "(\(first), \(second))"
    }
}
```

In Rust:
```Rust
impl<T: std::fmt::Display, U: std::fmt::Display> std::fmt::Display for Pair<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}
```

Well, Rust looks verbose, doesn't it? :D 

## Performance. What about it?

Rust uses monomorphization: it generates concrete implementations for each type at compile time, which is very efficient but leads to bigger binaries.

In Swift, it's a bit different.
* Value types often behave the same as in Rust.
* Generic constraints of functions may get specialized (but developers don't get control over it, unless applying some `@_` attributes).

### Existentials, boxes, opaque types

Swift has recently gotten opaque types and existential containers. So, developers have better control over dynamic vs static dispatch and performance costs.
Rust also has it in store.

Let's imagine we have a trait/protocol Pet, that has one function `talk`. 

In Rust, we can use `impl Trait` to use monomorphization to create a specialized instance of the function for each different type that the generic is instantiated with. This means that calling a trait method from within a generic function still uses static dispatch, as the compiler has full type information and can resolve which type’s trait implementation to use.
```Rust
trait Pet {
    fn talk(&self) -> String;
}

// Uses generics and static dispatch.
fn generic(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// It could have been like this, but there's more nuance to that
fn generic<PetType: Pet>(pet: &Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// Uses type-erasure and dynamic dispatch.
fn dynamic(pet: &dyn Pet) {
    println!("Hello, who are you? {}", pet.talk());
}
```

In Swift, it would look like this:

```Swift
protocol Pet {
    func talk() -> String
}

// Uses opaque type, static dispatch.
func generic(pet: some Pet) {
    print("Hello, who are you? \(pet.talk)")
}

// Could have been like this, but there's more nuance to that
func generic<PetType: Pet>(pet: PetType) {
    print("Hello, who are you? \(pet.talk)")
}

// Existential box, dynamic dyspatch
func dynamic(pet: any Pet) {
    print("Hello, who are you? \(pet.talk)")
}
```

As a rule of thumb, think about any vs some and dyn vs impl. 

`any` and `dyn` maybe the simplest way to write code, but it comes with the cost.

`some` in Swift is great for return types, but for parameters it may be more convenient to use good old <ExplicitGeneric>.

In Rust, `impl` is a cool shorthand syntax, but if you need flexibility and more powerful generics, use <ExplicitGeneric>.

That's all for today :) It's not the super deep article, but it was fun to write, because these two languages feel very close together. Soon, this feeling may change as I'm going into memory management in Rust.
