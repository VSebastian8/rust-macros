#[macro_export]
macro_rules! timed {
    ( $message: expr => $code: block ) => {{
        use std::time::SystemTime;
        let start = SystemTime::now();
        // Run the provided block of code
        let res = $code;
        // Calculate the time difference
        let end = SystemTime::now();
        let diff = end.duration_since(start).expect("std::time error");
        // Show the message
        println!("{} took: {} ms", stringify!($message), diff.as_millis());
        // Return the result of the code block
        res
    }};
}
