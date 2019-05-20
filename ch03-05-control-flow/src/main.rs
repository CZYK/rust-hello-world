fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }



    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);



    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);



    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");



    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }



    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }


    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");


    let fahrenheit: i32 = 100;
    let celsius = convert_to_celsius(fahrenheit);
    println!("celsius: {}", celsius);

    let fahrenheit = convert_to_fahrenheit(celsius);
    println!("fahrenheit: {}", fahrenheit);

//    let fibonacci = generate_nth_fibonacci(50);
//    println!("fibonacci: {}", fibonacci);

//    let fibonacci = fib(50);
//    println!("fibonacci: {}", fibonacci);

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(10).take(50) {
        println!("> {}", i);
    }
}


fn convert_to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

fn convert_to_fahrenheit(celsius: i32) -> i32 {
    (celsius * 9/5) + 32
}

fn generate_nth_fibonacci(nth: usize) -> u64 {

    let mut sequence = vec![0, 1];

    for number in 2..nth + 1 {
        let a = sequence[number - 1];
        let b = sequence[number - 2];
        sequence.push(a + b);
    }

    sequence[nth]
}


fn fib(n: u64) -> u64 {

    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

struct Fibonacci {
    curr: u64,
    next: u64,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    type Item = u64;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}