pub mod private;
pub mod public;
pub mod structs;
pub mod types;
mod util;
pub mod wsfeed;

pub use private::Private;
pub use public::Public;

pub const GEMINI_SANDBOX_URL: &str = "https://api.sandbox.gemini.com";
pub const GEMINI_SANDBOX_WS_URL: &str = "wss://api.sandbox.gemini.com";
pub const GEMINI_MAIN_URL: &str = "https://api.gemini.com";
pub const GEMINI_WS_URL: &str = "wss://api.gemini.com";
