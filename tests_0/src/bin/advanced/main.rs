use core::fmt;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    fs::File,
    io::Read,
};

fn read() -> Result<String, std::io::Error> {
    let mut file = File::open("text.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

#[derive(Debug)]
enum LoginError {
    InvalidUsername,
    InvalidPassword,
}

fn login(user: &str, pass: &str) -> Result<(), LoginError> {
    if user != "admin" {
        Err(LoginError::InvalidUsername)
    } else if pass != "1234" {
        Err(LoginError::InvalidPassword)
    } else {
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.age.cmp(&other.age) {
            Ordering::Equal => self.name.cmp(&other.name),
            ord => ord,
        }
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let a = 0b011;
    let x = a << 1;
    println!("Answer is {}", x);

    let s = "Hello";
    println!("{}", s.chars().nth(1).unwrap());

    let mut s = String::from("rust");
    {
        let r1 = &mut s;
        r1.push_str("!?");
        println!("{}", r1);
    }
    let r2 = &mut s;
    r2.push('!');
    println!("{} and {}", r2, r2);

    for count in (0..=4).rev().step_by(2) {
        println!("Count is {}", count);
    }
    println!("End for");

    'outer: for i in 0..3 {
        for j in 10..13 {
            if i == 1 && j == 11 {
                continue 'outer;
            }
            println!("Outer: {i}, Inner: {j}");
        }
    }

    let mut x = 0;
    let first_square_above_20 = loop {
        x += 1;
        if (x * x) > 20 {
            break x;
        }
    };
    println!("Result: {first_square_above_20}");

    println!("{}", read().unwrap());

    let login = login("admin", "12345");
    match login {
        Ok(_) => println!("Login successful!"),
        Err(e) => println!("Fail: {:?}", e),
    }

    let s = String::from("This is ordinary sentence");
    let mut hashmap: HashMap<char, usize> = HashMap::new();
    for c in s.to_lowercase().chars() {
        *hashmap.entry(c).or_insert(0) += 1;
    }
    println!("{:?}", hashmap);

    let mut contacts: BTreeMap<&str, i32> = BTreeMap::new();
    contacts.insert("Lotty", 10);
    contacts.insert("Fila", 10);
    contacts.insert("Tifa", 5);
    contacts.insert("Lucy", 20);
    println!("{:?}", contacts);

    let mut events = BTreeMap::new();
    events.insert((2026, 1, 1), "Jan 1");
    events.insert((2025, 12, 31), "Dec 31");
    events.insert((2026, 1, 2), "Jan 2");
    for ((y, m, d), event) in &events {
        println!("{:04}-{:02}-{:02}: {}", y, m, d, event);
    }

    let mut people = BTreeMap::new();
    people.insert(
        Person {
            name: "Lotty".to_string(),
            age: 30,
        },
        "Engineer",
    );
    people.insert(
        Person {
            name: "Lucy".to_string(),
            age: 25,
        },
        "Designer",
    );
    people.insert(
        Person {
            name: "Fila".to_string(),
            age: 34,
        },
        "Manager",
    );
    for (person, role) in &people {
        println!("{:?} -> {}", person, role);
    }

    let mut map1 = BTreeMap::new();
    map1.insert(1, "a");
    map1.insert(2, "b");
    map1.insert(3, "c");
    let mut map2 = BTreeMap::new();
    map2.insert(3, "C");
    map2.insert(4, "d");
    println!("\nBefore append: {:?} | {:?}", map1, map2);
    map1.append(&mut map2);
    println!("\nAfter append: {:?} | {:?}", map1, map2);
    let map3 = map1.split_off(&3);
    println!("\nAfter split_off(3): {:?} | {:?}", map1, map3);

    let mut fruits = BTreeMap::new();
    fruits.insert("apple", 10);
    fruits.insert("banana", 20);
    fruits.insert("cherry", 30);
    fruits.insert("date", 40);
    println!("=== Fruits A-C ===");
    for (name, count) in fruits.range_mut("apple".."date") {
        *count += 5;
        println!("{}: {}kg", name, count);
    }

    let mut set1: HashSet<i32> = HashSet::new();
    set1.insert(10);
    set1.insert(20);
    set1.insert(30);
    let mut set2: HashSet<i32> = HashSet::new();
    set2.insert(20);
    set2.insert(30);
    set2.insert(40);
    println!("Disjoint: {:?}", set1.is_disjoint(&set2));
    println!("Union: {:?}", set1.union(&set2));
    println!("Intersection: {:?}", set1.intersection(&set2));
    println!("Difference: {:?}", set1.difference(&set2));
    println!("Subset: {:?}", set1.is_subset(&set2));
    println!("Superset: {:?}", set1.is_superset(&set2));

    let mut set: BTreeSet<i32> = BTreeSet::new();
    set.insert(30);
    set.insert(20);
    set.insert(40);
    set.insert(10);
    println!("{:?}", set);

    let mut heap = BinaryHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(9);
    heap.push(2);
    heap.push(4);
    heap.push(10);
    println!("Binary Heap: {:?}", heap);
    if let Some(mut val) = heap.peek_mut() {
        println!("Top priority: {:?} (before)", val);
        *val -= 2;
    }
    println!("Binary Heap: {:?}", heap);

    println!("{}", add(5, 3));

    let p1: Point<i32> = Point { x: 1, y: 2 };
    let p2: Point<i32> = Point { x: 3, y: 4 };
    let p3 = p1.add(p2);
    println!("{:?}", p3);

    let arr: [i32; 5] = [2, 6, 3, 7, 5];
    print_array(arr);

    // let dog = Dog;
    // let cat = Cat;
    // let bird = Bird;
    // let animals: Vec<&dyn Sound> = vec![&dog, &cat, &bird];
    // for animal in animals {
    //     println!("{}", animal.make_sound());
    // }
    // let shark = Shark;
    // let _: Vec<&dyn Animal> = vec![&dog, &cat, &bird, &shark];

    let nums = Numbers(71, 100);
    let words = Words("hello".to_string(), "world".to_string());
    println!("First number: {}", nums.first());
    println!("First word: {}", words.first());

    let dog = GoldenRetriever {
        name: "Rex".to_string(),
    };
    println!("Name: {}", dog.name());
    println!("Legs: {}", dog.legs());

    let point = Point { x: 1, y: 2 };
    let user = User {
        name: "Alice".to_string(),
    };
    println!("{}", point);
    println!("{}", user);
    println!("{}", 42u64);
    println!("{}", point.get_value());
    println!("{}", user.get_value());
    println!("{}", 42u64.get_value());

    let stack_num: i32 = 51;
    let boxed_num: Box<i32> = Box::new(51);
    let bexed_array: Box<[i32; 1000]> = Box::new([0; 1000]);
    println!("{} bytes", size_of_val(&stack_num));
    println!("{} bytes", size_of_val(&boxed_num));
    println!("{} bytes", size_of_val(&bexed_array));
    println!("{} bytes", size_of::<i32>());
    println!("{} bytes", size_of::<[i32; 1000]>());
    println!("{}", stack_num);
    println!("{}", *boxed_num);
    println!("{:?}", *bexed_array);

    let small_list = get_numbers(2);
    let large_list = get_numbers(3);
    println!("{:?} {:?}", small_list, large_list);

    let mut b: Box<String> = Box::default();
    println!("Initial empty value: '{}'", b);
    b.push_str("Hello from the heap!");
    println!("Updated value: '{}'", b);

    let slice: &str = "Hello, world!";
    let boxed_slice: String = slice.into();
    println!("{}", boxed_slice);
    println!("{}", slice);

    let another: String = String::from("Rust String");
    let boxed_string: Box<str> = another.into();
    println!("{}", boxed_string);

    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            length: 10.0,
            breadth: 4.0,
        }),
    ];
    for shape in shapes {
        shape.draw();
    }

    let get_square = |n| n * n;
    println!("{}", get_square(5));
    let add = |a, b| a + b;
    println!("{}", add(10, 45));
    let length = |s: &str| s.len();
    println!("{}", length("rust!"));

    let a = 36;
    let b = 4;
    fn addition(a: i32, b: i32) -> i32 {
        a + b
    }
    fn subtraction(a: i32, b: i32) -> i32 {
        a - b
    }
    println!("{}", addition(a, b));
    println!("{}", subtraction(a, b));
    let add = || a + b;
    let sub = || a - b;
    println!("\n{}", add());
    println!("{}", sub());

    let mut count = 0;
    let mut increment = || {
        println!("{}", count);
        count += 1;
    };
    increment();
    increment();
    println!("{}", count);

    let mut name = String::from("Hello");
    let mut print_name = move || {
        println!("{name}");
        name.push_str(", world!");
    };
    print_name();
    // println!("{}", name);
    print_name();

    let greeting = String::from("Hello");
    let print_greet = || println!("{}", greeting);
    call_with_fn(print_greet);

    let mut counter = 0;
    let increment = || {
        counter += 1;
        println!("Counter: {}", counter);
    };
    call_with_fn_mut(increment);

    let heavy_data = String::from("Massive String!");
    let consume_data = || {
        println!("{} consumed", heavy_data);
    };
    call_with_fn_once(consume_data);

    let remainder = modulus();
    println!("{}", remainder(10, 3));

    let mut numbers = vec![9, 2, 5, 3, 6, 1, 7, 0, 8, 4, 2];
    numbers.sort_by(|a, b| a.cmp(b));
    println!("Num={:?}", numbers);
    let increased: Vec<i32> = numbers.iter().map(|x| x + 1).collect();
    println!("Inc={:?}", increased);
    let even: Vec<&i32> = increased.iter().filter(|&x| x % 2 == 0).collect();
    println!("Even={:?}", even);
    let sum = even.iter().fold(0, |acc, &x| acc + x);
    println!("Sum={:?}", sum)
}

fn modulus() -> impl Fn(i32, i32) -> i32 {
    |x, y| x % y
}

fn call_with_fn_once<F>(func: F)
where
    F: FnOnce(),
{
    func();
}

fn call_with_fn_mut<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

fn call_with_fn<F>(func: F)
where
    F: Fn(),
{
    func();
    func();
}

trait Draw {
    fn draw(&self);
}
struct Circle {
    radius: f64,
}
struct Rectangle {
    length: f64,
    breadth: f64,
}
impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius: {}", self.radius);
    }
}
impl Draw for Rectangle {
    fn draw(&self) {
        println!(
            "Drawing rectangle with length: {} and breadth: {}",
            self.length, self.breadth
        );
    }
}

fn get_numbers(count: usize) -> Box<[i32]> {
    if count == 2 {
        Box::new([1, 2])
    } else {
        Box::new([1, 2, 3])
    }
}

trait Value {
    fn get_value(&self) -> String;
}
impl<T> Value for T
where
    T: std::fmt::Display + std::fmt::Debug,
{
    fn get_value(&self) -> String {
        format!("Value is {}", self)
    }
}
#[derive(Debug)]
struct User {
    name: String,
}
impl<T: std::fmt::Debug> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

trait Pet {
    fn name(&self) -> String;
}
trait Dog: Pet {
    fn legs(&self) -> u32;
}
struct GoldenRetriever {
    name: String,
}
impl Dog for GoldenRetriever {
    fn legs(&self) -> u32 {
        4
    }
}
impl Pet for GoldenRetriever {
    fn name(&self) -> String {
        self.name.clone()
    }
}

trait Container {
    type Item;
    fn first(&self) -> Self::Item;
}
struct Numbers(i32, i32);
impl Container for Numbers {
    type Item = i32;

    fn first(&self) -> Self::Item {
        self.0
    }
}
struct Words(String, String);
impl Container for Words {
    type Item = String;

    fn first(&self) -> Self::Item {
        self.0.clone()
    }
}

fn print_array<const N: usize>(array: [i32; N]) {
    for a in array {
        println!("{}", a);
    }
}

trait Sound {
    fn make_sound(&self) -> &str;
}

// struct Dog;
// struct Cat;
// struct Bird;
// struct Shark;
//
// impl Sound for Dog {
//     fn make_sound(&self) -> &str {
//         "Woof!"
//     }
// }
//
// impl Sound for Cat {
//     fn make_sound(&self) -> &str {
//         "Meow!"
//     }
// }
//
// impl Sound for Bird {
//     fn make_sound(&self) -> &str {
//         "Chirp!"
//     }
// }
//
// trait Animal {}
// impl Animal for Dog {}
// impl Animal for Cat {}
// impl Animal for Bird {}
// impl Animal for Shark {}
//
// fn get_animal_sound<T: Sound + Animal>(animal: &T) -> &str {
//     animal.make_sound()
// }

// fn get_animal_sound(animal: &impl Sound) -> &str {
//     animal.make_sound()
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::ops::Add<Output = T>> Point<T> {
    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
