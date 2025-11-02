# rust-macros

### Example 1: `timed!`

**Declarative macro** that measures how long a block of code takes to run

```rust
timed! {
    2 * 3 =>
    {
        println!("Multiplication result: {}", 2 * 3)
    }
}
```

> Multiplication result: 6 \
> 2 \* 3 took: 0 ms

```rust
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
```

> Calculating the sum \
> Done \
> "vector sum" took: 5 ms \
> Sum: 1000000

### Example 2
