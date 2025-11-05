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

Macro Code - mtimed/src/lib.rs

### Example 2: `comp!`

**Function-Like Procedural macro** that implements a simplified list comprehension from Python

```rust
let v: Vec<i32> = comp![x * 2 for x <- [1, 2, 3]].collect();
println!("{:?}", v);
```

> [2, 4, 6]

```rust
let nums = vec![12, 13, 15, 16, 17];
let evens: Vec<bool> = comp![num % 2 == 0 for num <- nums].collect();
println!(":?", evens)
```

> [true, false, false, true, false]

```rust
let sums = comp![x + y for (x, y) <- [(1, 2), (3, 4), (5, 6)]];
println!("Sums: {:?}", sums.collect::<Vec<i32>>());
```

> Sums: [3, 7, 11]

Macro Code - comp/comp_macro/src/lib.rs
