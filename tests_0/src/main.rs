use std::rc::Rc;

fn with_logging<F>(inner: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    move |value| {
        println!("[LOG] Calling function with {}", value);
        let result = inner(value);
        println!("[LOG] Function returned {}", result);
        result
    }
}

fn with_timing<F>(inner: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    move |value| {
        let start = std::time::Instant::now();
        let result = inner(value);
        let elapsed = start.elapsed();
        println!("[TIMING] Took {:?}", elapsed);
        result
    }
}

fn with_counting<F>(inner: F) -> impl FnMut(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    let mut count = 0;
    move |value| {
        count += 1;
        println!("[COUNT] Call #{} with value {}", count, value);
        inner(value)
    }
}

fn with_caching<F>(inner: F) -> impl FnMut(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    let mut cache = std::collections::HashMap::new();
    move |value| {
        if let Some(&result) = cache.get(&value) {
            println!("[CACHE] Hit for {} -> {}", value, result);
            return result;
        }
        let result = inner(value);
        println!("[CACHE] Miss for {} -> {}", value, result);
        cache.insert(value, result);
        result
    }
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let add_one = |x: i32| x + 1;
    let add_one_logged = with_logging(add_one);
    let result = add_one_logged(5);
    println!("Final result: {}", result);

    let slow_square = |x: i32| {
        std::thread::sleep(std::time::Duration::from_millis(50));
        x * x
    };
    let timed_square = with_timing(slow_square);
    let r1 = timed_square(4);
    let r2 = timed_square(7);
    println!("Results: {} and {}", r1, r2);

    let increment = |x: i32| x + 1;
    let mut counted_inc = with_counting(increment);
    counted_inc(10);
    counted_inc(20);
    counted_inc(30);
    let final_result = counted_inc(40);
    println!("Final result: {}", final_result);

    let double = |x: i32| x * 2;
    let pipeline = with_logging(with_timing(double));
    let result = pipeline(5);
    println!("Final result: {}", result);

    let expensive = |x: i32| {
        println!("(Computing {}...)", x);
        std::thread::sleep(std::time::Duration::from_millis(100));
        x * x
    };
    let mut cached = with_caching(expensive);
    println!("First call with 5:");
    let r1 = cached(5);
    println!("Second call with 5:");
    let r2 = cached(5);
    println!("Third call with 10:");
    let r3 = cached(10);
    println!("Fourth call with 5:");
    let r4 = cached(5);
    println!("\nResults: {} {} {} {}", r1, r2, r3, r4);

    let numbers = Vec::from([1, 2, 3]);
    let mut numbers_iter = numbers.iter();
    let total: i32 = numbers_iter.clone().sum();
    let mapped: Vec<i32> = numbers.iter().map(|x| x + 1).filter(|&x| x > 2).collect();
    dbg!(numbers_iter.next());
    dbg!(numbers_iter.next());
    dbg!(numbers_iter.next());
    dbg!(numbers_iter.next());
    dbg!(total);
    dbg!(mapped);

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Square { side: 3.0 }),
    ];
    for shape in shapes {
        println!("Area: {}", shape.area());
    }

    let tree: BinaryTree = BinaryTree::Node(
        1,
        Box::new(BinaryTree::Node(2, Box::new(leaf(4)), Box::new(leaf(5)))),
        Box::new(leaf(3)),
    );
    println!("Tree: {tree:?}");

    let d: Rc<Node> = Rc::new(Node {
        value: 4,
        neighbors: vec![],
    });
    let b: Rc<Node> = Rc::new(Node {
        value: 2,
        neighbors: vec![Rc::clone(&d)],
    });
    let c: Rc<Node> = Rc::new(Node {
        value: 3,
        neighbors: vec![Rc::clone(&d)],
    });
    let a: Rc<Node> = Rc::new(Node {
        value: 1,
        neighbors: vec![Rc::clone(&b), Rc::clone(&c)],
    });
    println!("D's owners: {}", Rc::strong_count(&d));
    println!("A: {a:?}");
}

#[derive(Debug)]
struct Node {
    value: i32,
    neighbors: Vec<Rc<Node>>,
}

#[derive(Debug)]
enum BinaryTree {
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
    Empty,
}

fn leaf(value: i32) -> BinaryTree {
    BinaryTree::Node(
        value,
        Box::new(BinaryTree::Empty),
        Box::new(BinaryTree::Empty),
    )
}
