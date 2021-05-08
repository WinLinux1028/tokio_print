#[macro_use]
extern crate lazy_static;

use safety_breaker::ForceMut;
use tokio::io::AsyncWriteExt;

lazy_static! {
    static ref STDOUT: tokio::io::Stdout = tokio::io::stdout();
    static ref STDERR: tokio::io::Stderr = tokio::io::stderr();
}

/// # Async version of println!
/// ## Example
/// ```
/// #[macro_use]
/// extern crate tokio_print;
///
/// #[tokio::main]
/// async fn main() {
///     aprintln!("Hello World!"); //Hello World!
/// }
/// ```
/// ## How to use
/// same to println!
#[macro_export]
macro_rules! aprintln {
    () => (tokio_print::aprint1("\n").await);
    ($fmt:expr) => (tokio_print::aprint1(concat!($fmt, "\n")).await);
    ($fmt:expr, $($arg:tt)*) => (tokio_print::aprint1(&format!(concat!($fmt, "\n"), $($arg)*)).await);
}

/// # Async version of eprintln!
/// ## Example
/// ```
/// #[macro_use]
/// extern crate tokio_print;
///
/// #[tokio::main]
/// async fn main() {
///     aeprintln!("Error!"); //Error!
/// }
/// ```
/// ## How to use
/// same to eprintln!
#[macro_export]
macro_rules! aeprintln {
    () => (tokio_print::aeprint1("\n").await);
    ($fmt:expr) => (tokio_print::aeprint1(concat!($fmt, "\n")).await);
    ($fmt:expr, $($arg:tt)*) => (tokio_print::aeprint1(&format!(concat!($fmt, "\n"), $($arg)*)).await);
}

/// # Async version of print!
/// ## Example
/// ```
/// #[macro_use]
/// extern crate tokio_print;
///
/// #[tokio::main]
/// async fn main() {
///     aprintln!("Hello World!"); //Hello World!
/// }
/// ```
/// ## How to use
/// same to println!
#[macro_export]
macro_rules! aprint {
    () => (());
    ($fmt:expr) => (tokio_print::aprint1($fmt).await);
    ($fmt:expr, $($arg:tt)*) => (tokio_print::aprint1(&format!($fmt, $($arg)*)).await);
}

/// # Async version of eprint!
/// ## Example
/// ```
/// #[macro_use]
/// extern crate tokio_print;
///
/// #[tokio::main]
/// async fn main() {
///     aeprintln!("Error!"); //Error!
/// }
/// ```
/// ## How to use
/// same to eprint!
#[macro_export]
macro_rules! aeprint {
    () => (());
    ($fmt:expr) => (tokio_print::aeprint1($fmt).await);
    ($fmt:expr, $($arg:tt)*) => (tokio_print::aeprint1(&format!($fmt, $($arg)*)).await);
}

#[inline(always)]
pub async fn aprint1(content: &str) {
    unsafe {
        (*STDOUT)
            .forcemut()
            .write_all(content.as_bytes())
            .await
            .unwrap();
    }
}

#[inline(always)]
pub async fn aeprint1(content: &str) {
    unsafe {
        (*STDERR)
            .forcemut()
            .write_all(content.as_bytes())
            .await
            .unwrap();
    }
}
