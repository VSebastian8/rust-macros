#[macro_export]
macro_rules! timed {
    ( $message: expr => $code: block ) => {{
        use std::time::SystemTime;
        // Measure the time before
        let start = SystemTime::now();
        // Run the provided block of code
        let res = $code;
        // Measure the time after
        let end = SystemTime::now();
        // Calculate the time difference
        let diff = end.duration_since(start).expect("std::time error");
        // Show the message
        println!("{} took: {} ms", stringify!($message), diff.as_millis());
        // Return the result of the code block
        res
    }};
}
