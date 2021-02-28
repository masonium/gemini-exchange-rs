pub mod public;
pub mod private;
mod util;
pub mod structs;
pub mod types;
pub mod wsfeed;

pub use public::Public;
pub use private::Private;


pub const GEMINI_SANDBOX_URI: &'static str = "https://api.sandbox.gemini.com";
pub const GEMINI_SANDBOX_WS_URI: &'static str = "wss://api.sandbox.gemini.com";
