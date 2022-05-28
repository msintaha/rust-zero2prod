use std::env;

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", &self.first_name, &self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    let sliced_arr: &[i32] = &numbers[0..2];

    println!("{:?}", sliced_arr);

    let mut vector_nums: Vec<i32> = vec![1,3,4,5];

    println!("{:?}", vector_nums);

    for i in vector_nums.iter(){
        println!("Woah {}", i);
    }

    for x in vector_nums.iter_mut() {
        *x *= 2;
    }

    println!("mutated {:?}", vector_nums);

    // let mut count = 0;
    for count in 1..5 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        }
    }

    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("added {}", add_nums(4, 5));
    
    let mut p = Person::new("Peter", "Stewie");
    p.set_last_name("Griffin");
    println!("struct {}", p.full_name());
    println!("tuple {}", p.to_tuple().0);

    enum Movement {
        Up,
        Down,
        Left,
        Right
    }

    fn move_avatar(m: Movement) {
        match m {
            Movement::Left => println!("Moving left"),
            Movement::Right => println!("Moving right"),
            Movement::Up => println!("Moving up"),
            Movement::Down => println!("Moving down"),
        };
    }

    move_avatar(Movement::Left);
    move_avatar(Movement::Right);
    move_avatar(Movement::Up);
    move_avatar(Movement::Down);

    let args: Vec<String> = env::args().collect();
    println!("cmd {:?}", args[1].clone());
}