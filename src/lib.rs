#[macro_export]
macro_rules! try_log {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error at {}:{} - {:?}", file!(), line!(), err);
                return Err(err.to_string());
            }
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_log {
    ($expr:expr, $default:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                eprintln!(
                    "Unwrap failed at {}:{} - {:?}. Using default: {:?}",
                    file!(),
                    line!(),
                    err,
                    $default
                );
                $default
            }
        }
    };
}

#[macro_export]
macro_rules! time_it {
    ($label:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = { $block };
        let duration = start.elapsed();
        println!("{} took {:?}", $label, duration);
        result
    }};
}

#[macro_export]
macro_rules! json_merge {
    ($base:expr, $other:expr) => {{
        let mut base = $base;
        if let (Some(base_obj), Some(other_obj)) = (base.as_object_mut(), $other.as_object()) {
            for (k, v) in other_obj {
                base_obj.insert(k.clone(), v.clone());
            }
        }
        base
    }};
}

#[macro_export]
macro_rules! debug_query {
    ($query:expr) => {{
        let sql = $query.sql();
        println!("SQL Query: {}", sql);
        $query
    }};
}

#[macro_export]
macro_rules! with_retry {
    ($retries:expr, $delay_ms:expr, $expr:expr) => {{
        let mut attempts = 0;
        loop {
            match $expr {
                Ok(val) => break Ok(val),
                Err(err) => {
                    attempts += 1;
                    if attempts >= $retries {
                        break Err(err);
                    }
                    std::thread::sleep(std::time::Duration::from_millis($delay_ms));
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! retry_async {
    ($retries:expr, $delay_ms:expr, $async_expr:expr) => {{
        use std::time::Duration;
        let mut attempts = 0;
        loop {
            match $async_expr.await {
                Ok(val) => break Ok(val),
                Err(err) => {
                    attempts += 1;
                    if attempts >= $retries {
                        break Err(err);
                    }
                    tokio::time::sleep(Duration::from_millis($delay_ms)).await;
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! span_wrap {
    ($span_name:expr, $block:block) => {{
        let span = tracing::span!(tracing::Level::INFO, $span_name);
        let _enter = span.enter();
        $block
    }};
}

#[macro_export]
macro_rules! log_duration {
    ($label:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = { $block };
        let elapsed = start.elapsed();
        tracing::info!("{} took {:?}", $label, elapsed);
        result
    }};
}

#[macro_export]
macro_rules! call_with_trace {
    ($span_name:expr, $func:expr $(, $args:expr)*) => {{
        let span = tracing::span!(tracing::Level::INFO, $span_name);
        let _enter = span.enter();
        $func($($args),*)
    }};
}

#[macro_export]
macro_rules! assert_msg {
    ($cond:expr, $msg:expr) => {
        if !$cond {
            tracing::error!("Assertion failed: {}", $msg);
            panic!($msg);
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($expr:expr, $default:expr) => {{
        match $expr {
            Ok(val) => val,
            Err(err) => {
                tracing::error!("Error: {:?}", err);
                $default
            }
        }
    }};
}

#[macro_export]
macro_rules! parse_env {
    ($var:expr, $default:expr) => {{
        std::env::var($var).unwrap_or_else(|_| {
            tracing::warn!(
                "Environment variable {} not set. Using default: {:?}",
                $var,
                $default
            );
            $default.to_string()
        })
    }};
}

#[macro_export]
macro_rules! pretty_debug {
    ($obj:expr) => {
        println!("{}", serde_json::to_string_pretty(&$obj).unwrap())
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::env;
    use std::error::Error;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    // Test try_log! with a successful result.
    #[test]
    fn test_try_log_ok() {
        fn test_fn() -> Result<i32, String> {
            let x = try_log!(Ok::<_, Box<dyn Error>>(10));
            Ok(x)
        }
        assert_eq!(test_fn().unwrap(), 10);
    }

    // Test try_log! when an error occurs. It should return early.
    #[test]
    fn test_try_log_err() {
        fn test_fn() -> Result<i32, String> {
            // This will trigger the error branch in try_log!.
            let _x = try_log!(Err("error".to_string()));
            // This line should never be reached.
            Ok(42)
        }
        let res = test_fn();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "error".to_string());
    }

    // Test unwrap_or_log! macro.
    #[test]
    fn test_unwrap_or_log() {
        let ok_val: Result<&str, &str> = Ok("hello");
        let err_val: Result<&str, &str> = Err("fail");
        let v1 = unwrap_or_log!(ok_val, "default");
        assert_eq!(v1, "hello");
        let v2 = unwrap_or_log!(err_val, "default");
        assert_eq!(v2, "default");
    }

    // Test time_it! macro.
    #[test]
    fn test_time_it() {
        let result = time_it!("sleep test", {
            std::thread::sleep(Duration::from_millis(50));
            5
        });
        assert_eq!(result, 5);
    }

    // Test json_merge! macro.
    #[test]
    fn test_json_merge() {
        let base = json!({"a": 1, "b": 2});
        let other = json!({"b": 3, "c": 4});
        let merged = json_merge!(base, other);
        assert_eq!(merged["a"], 1);
        assert_eq!(merged["b"], 3);
        assert_eq!(merged["c"], 4);
    }

    // For debug_query!, create a dummy type with a .sql() method.
    struct DummyQuery {
        sql: String,
    }
    impl DummyQuery {
        fn new(sql: &str) -> Self {
            DummyQuery {
                sql: sql.to_string(),
            }
        }
        fn sql(&self) -> &str {
            &self.sql
        }
    }
    #[test]
    fn test_debug_query() {
        let query = DummyQuery::new("SELECT 1");
        let _ = debug_query!(query);
        // The macro prints the SQL; we simply ensure it does not panic.
    }

    // Test with_retry! macro.
    #[test]
    fn test_with_retry_success() {
        static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
        let res = with_retry!(3, 10, {
            let current = ATTEMPTS.fetch_add(1, Ordering::SeqCst);
            if current < 2 {
                Err("fail")
            } else {
                Ok("success")
            }
        });
        assert_eq!(res.unwrap(), "success");
    }

    #[test]
    fn test_with_retry_failure() {
        let res: Result<&str, &str> = with_retry!(2, 10, { Err("always fails") });
        assert!(res.is_err());
    }

    // Test retry_async! macro.
    #[tokio::test]
    async fn test_retry_async_success() {
        use std::sync::Arc;
        use tokio::sync::Mutex;
        let attempts = Arc::new(Mutex::new(0));
        let res = retry_async!(3, 10, {
            let attempts = attempts.clone();
            async move {
                let mut att = attempts.lock().await;
                if *att < 2 {
                    *att += 1;
                    Err("fail")
                } else {
                    Ok("success")
                }
            }
        });
        assert_eq!(res.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_retry_async_failure() {
        let res: Result<&str, &str> = retry_async!(2, 10, async { Err("fail") });
        assert!(res.is_err());
    }

    // Test span_wrap! macro.
    #[test]
    fn test_span_wrap() {
        let value = span_wrap!("test_span", { 123 });
        assert_eq!(value, 123);
    }

    // Test log_duration! macro.
    #[test]
    fn test_log_duration() {
        let value = log_duration!("duration test", { 456 });
        assert_eq!(value, 456);
    }

    // Test call_with_trace! macro.
    #[test]
    fn test_call_with_trace() {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        let result = call_with_trace!("add", add, 3, 4);
        assert_eq!(result, 7);
    }

    // Test assert_msg! macro. This test expects a panic.
    #[test]
    #[should_panic(expected = "Assertion failed: test failure")]
    fn test_assert_msg() {
        assert_msg!(false, "Assertion failed: test failure");
    }

    // Test log_error! macro.
    #[test]
    fn test_log_error() {
        let ok_val: Result<&str, &str> = Ok("ok");
        let err_val: Result<&str, &str> = Err("error");
        let v1 = log_error!(ok_val, "default");
        assert_eq!(v1, "ok");
        let v2 = log_error!(err_val, "default");
        assert_eq!(v2, "default");
    }

    // Test parse_env! macro.
    #[test]
    fn test_parse_env() {
        // Set an environment variable temporarily.
        unsafe {
            env::set_var("TEST_VAR", "value1");
        }
        let result = parse_env!("TEST_VAR", "default");
        assert_eq!(result, "value1".to_string());
        unsafe {
            env::remove_var("TEST_VAR");
        }

        // Now TEST_VAR is not set, so we get the default.
        let result = parse_env!("TEST_VAR", "default");
        assert_eq!(result, "default".to_string());
    }

    // Test pretty_debug! macro.
    #[test]
    fn test_pretty_debug() {
        let obj = json!({"x": 1, "y": 2});
        // Call the macro to ensure it doesn't panic.
        pretty_debug!(obj);
    }
}
