use mtimed::timed;

fn main() {
    timed! {
        2 * 3 =>
        {
            println!("Multiplication result: {}", 2 * 3)
        }
    }
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

    println!("Sum: {}", nums_sum);
}
