#[macro_export]
macro_rules! try_log {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error at {}:{} - {:?}", file!(), line!(), err);
                return Err(err.into());
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
                eprintln!("Unwrap failed at {}:{} - {:?}. Using default: {:?}", file!(), line!(), err, $default);
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
            tracing::warn!("Environment variable {} not set. Using default: {:?}", $var, $default);
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
