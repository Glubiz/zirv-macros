# zirv-macros

**zirv-macros** is a library of custom macros designed to make backend development in Rust easier, particularly for projects using Actix and SQLx. This library provides macros for error handling, logging, instrumentation, environment parsing, JSON manipulation, and more.

## Features

- **Error Handling & Assertions:**
  - `try_log!`: Evaluates an expression returning a `Result`, logs on error, and returns an error.
  - `unwrap_or_log!`: Unwraps a result and uses a default if it fails, logging the error.
  - `assert_msg!`: Asserts a condition with a custom error message.

- **Timing & Instrumentation:**
  - `time_it!`: Measures and logs the execution time of a code block.
  - `log_duration!`: Logs the duration of a code block using tracing.
  - `span_wrap!`: Wraps a block of code in a tracing span.
  - `call_with_trace!`: Calls a function inside a tracing span.

- **JSON & Environment Helpers:**
  - `json_merge!`: Merges two JSON objects.
  - `parse_env!`: Reads an environment variable with a default fallback.
  - `pretty_debug!`: Prints a pretty JSON representation of a serializable object.

- **SQL Debugging:**
  - `debug_query!`: Logs the SQL query string before executing it.

- **Retry Utilities:**
  - `with_retry!`: Retries a synchronous expression.
  - `retry_async!`: Retries an asynchronous expression.

## Installation

Add **zirv-macros** as a dependency in your project's `Cargo.toml`:

```toml
[dependencies]
zirv-macros = "0.1.2"
```
