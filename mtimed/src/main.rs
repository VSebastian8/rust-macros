use mtimed::timed;

// Function we want to benchmark
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    timed! {
        2 * 3 =>
        {
            println!("Multiplication result: {}", 2 * 3)
        }
    }
    // Multiplication result: 6
    // 2 * 3 took: 0 ms
    println!();

    let nums = vec![1; 1000000];
    let nums_sum = timed! {
        "vector sum" =>
        {
            println!("Calculating the sum");
            let mut sum = 0;
            for num in nums {
                sum += num;
            }
            println!("Done");
            sum
        }
    };
    // Calculating the sum
    // Done
    // "vector sum" took: 4 ms
    println!("Sum: {}", nums_sum);
    // Sum: 1000000

    timed!(Fibonacci => { fibonacci(35) });
    // Fibonacci took: 63 ms

    // Declarative Macros
    // print!("Hello {}!", "world");
    // println!("{} {} <{}", "hello", "world", 3);
    // format!("{} * {} = {}", 2, 3, 6); // "2 * 3 = 6"
    // stringify!(2 * 3 = 6); // 2 * 3 = 6
    // vec!['l', 'm', 'n', 'o', 'p'];
    // vec!["abc"; 3]; // ["abc", "abc", "abc"]
    // assert!(2 + 2 == 4, "{} + {} != {}", 2, 2, 4);
    // assert_eq!(3 + 2, 4); // assertion left == right failed
    // panic!("error");

    // Vec Macro
    // let v = vec![1, 2, 3, 4, 5];

    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);
    // v.push(5);

    // vec!(6, 7, 8);
    // vec!{6, 7, 8};
}
