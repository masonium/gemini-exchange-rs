pub mod public;
pub mod private;
mod util;
pub mod structs;
pub mod types;
pub mod wsfeed;

pub use public::Public;
pub use private::Private;

pub const GEMINI_SANDBOX_URL: &'static str = "https://api.sandbox.gemini.com";
pub const GEMINI_SANDBOX_WS_URL: &'static str = "wss://api.sandbox.gemini.com";
pub const GEMINI_MAIN_URL: &'static str = "https://api.gemini.com";
pub const GEMINI_WS_URL: &'static str = "wss://api.gemini.com";
