//!
//! Hierarchical logging module. Shows inf, wrn, err, dbg messages in a stdout. 
//! Every new call of sec() macros creates a section or node in a messages list.
//! Section is removed when current block is ends. Example:
//! // this code snippet:
//! {
//!     sec!("This is a section");
//!     inf!("Message 1");
//!     sec!("Another section");
//!     inf!("Message 2");
//! }
//! inf!("Third message");
//! // produces this output:
//! // >[2022-02-11 14:19:00.460530] INFO [src\xxx.rs:xx     ] This is a section
//! // >[2022-02-11 14:19:00.460530] INFO [src\xxx.rs:xx     ]   Message 1
//! // >[2022-02-11 14:19:00.460530] INFO [src\xxx.rs:xx     ]     Another section
//! // >[2022-02-11 14:19:00.460530] INFO [src\xxx.rs:xx     ]       Message 2
//! // >[2022-02-11 14:19:00.460530] INFO [src\xxx.rs:xx     ] Third message
//!
use log::{*};
use flexi_logger::{DeferredNow, style};
use time::{format_description::FormatItem, macros::format_description as fmt};
///
/// Format of one log message
///
const MSG_FMT: &[FormatItem<'static>] = fmt!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]");
///
/// Amount of spaces in left padding for macroses (inf, err,...)
///
pub static LEFT_PADDING: usize = 2;
///
/// Global left indentation for log messages (inf, err,...)
///
pub static mut INDENT: usize = 0;
///
/// Structure to create hierarchical padding in log macroses. It used
/// only in this module, but should be public to have access from macroses
///
pub struct Pad {}
impl Drop for Pad { fn drop(&mut self) { unsafe { INDENT = INDENT.wrapping_sub(LEFT_PADDING); } } }
///
/// Init logger
///
pub fn init() {
    flexi_logger::Logger::try_with_env_or_str("debug").unwrap().format(log_format).start().unwrap(); // use %RUST_LOG% to set log level. e.g.: SET RUST_LOG=info
}
///
/// Starts a section and prints information message before it.
/// sec!("Section");
/// inf!("A Message");
/// // outputs:
/// // >[2022-02-11 14:19:00.460530] INFO [src\xxx.rs:xx     ] Section
/// // >[2022-02-11 14:19:00.460640] INFO [src\xxx.rs:xx     ]   A Message
///
#[macro_export] macro_rules! sec {
    ($($arg:tt)?) => (
        let _p = $crate::logger::Pad {};
        info!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), $($arg)?);
        unsafe { $crate::logger::INDENT = $crate::logger::INDENT.wrapping_add($crate::logger::LEFT_PADDING) }
    );
    ($arg0:literal, $($args:tt)+) => (
        let _p = $crate::logger::Pad {};
        info!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), format!($arg0, $($args)+));
        unsafe { $crate::logger::INDENT = $crate::logger::INDENT.wrapping_add($crate::logger::LEFT_PADDING) }
    );
}
///
/// Prints information message to stdout. Supports "{}" placeholders.
/// inf!("Log message");
/// inf!("{} Message", "This is my");
/// // outputs:
/// // >[2022-02-11 14:19:00.460530] INFO [src\xxx.rs:xx     ] Log message
/// // >[2022-02-11 14:19:00.460640] INFO [src\xxx.rs:xx     ] This is my Message
///
#[macro_export] macro_rules! inf {
    ($($arg:tt)?) => (
        info!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), $($arg)?)
    );
    ($arg0:literal, $($args:tt)+) => (
        info!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), format!($arg0, $($args)+))
    );
}
///
/// Prints error message to stdout. Supports "{}" placeholders.
/// err!("Error message");
/// err!("{} Error", "This is my");
/// // outputs:
/// // >[2022-02-11 14:19:00.460530] ERR  [src\xxx.rs:xx     ] Error message
/// // >[2022-02-11 14:19:00.460640] ERR  [src\xxx.rs:xx     ] This is my Error
///
#[macro_export] macro_rules! err {
    ($($arg:tt)?) => (
        error!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), $($arg)?)
    );
    ($arg0:literal, $($args:tt)+) => (
        error!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), format!($arg0, $($args)+))
    );
}
///
/// Prints warning message to stdout. Supports "{}" placeholders.
/// wrn!("Warning message");
/// wrn!("{} Warning", "This is my");
/// // outputs:
/// // >[2022-02-11 14:19:00.460530] WARN [src\xxx.rs:xx     ] Warning message
/// // >[2022-02-11 14:19:00.460640] WARN [src\xxx.rs:xx     ] This is my Warning
///
#[macro_export] macro_rules! wrn {
    ($($arg:tt)?) => (
        warn!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), $($arg)?)
    );
    ($arg0:literal, $($args:tt)+) => (
        warn!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), format!($arg0, $($args)+))
    );
}
///
/// Prints debug message to stdout. Supports "{}" placeholders.
/// dbg!("Debug message");
/// dbg!("{} Debug", "This is my");
/// // outputs:
/// // >[2022-02-11 14:19:00.460530] DBG  [src\xxx.rs:xx     ] Debug message
/// // >[2022-02-11 14:19:00.460640] DBG  [src\xxx.rs:xx     ] This is my Debug
///
#[macro_export] macro_rules! dbg {
    ($($arg:tt)?) => (
        debug!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), $($arg)?)
    );
    ($arg0:literal, $($args:tt)+) => (
        debug!("{}{}", " ".repeat(unsafe { $crate::logger::INDENT }), format!($arg0, $($args)+))
    );
}
///
/// One log message format function
///
fn log_format(w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "[{}] {} [{:<24}] {}",
        style(level).paint(now.format(MSG_FMT)),
        style(level).paint(if level == Level::Error { "ERR ".to_string() } else if level == Level::Debug { "DBG ".to_string() } else { level.to_string() }),
        record.file().unwrap_or("<unnamed>").to_string() + ":" + &record.line().unwrap_or(0).to_string(),
        style(level).paint(&record.args().to_string())
    )
}