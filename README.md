# gemini-exchange-rs

`gemini-exchange-rs` is a Rust library for interacting with the
[Gemini](https://www.gemini.com) cryptocurrency exchange. This library
is heavily inspired by the excellent
[`coinbase-pro-rs`](https://github.com/inv2004/coinbase-pro-rs)
library by [inv2004](https://github.com/inv2004), which serves a
similar purpose for the Coinbase Pro Exchange.

This is library is nowhere near feature complete, and will likely
never be. However, it does support the basic public and private REST
APIs and will support the public and private Websocket feeds in the
near future. It should be relatively easy to add endpoints for those
that are not yet available.

## Design Points

Some non-obvious design decisions:
 - The library is async-only. This could change in the future, but
   part of my motivation for building this library is as a learning
   exercise for `tokio`, `hyper`, and other supporting libraries. I
   would probably welcome changes that would add synchronous support,
   but I am unlikely to develop them myself.
 - Prices (denominated in fiat currency) and amounts/sizes
   (denominated in cryptocurrencies of various kinds) are always kept
   as `String`s, rather than converting to `f64` or some other decimal
   notation. There are two good reasons for this.
	 1. Even though there is no arithmetic done inside the library,
		any user should be aware that any non-trivial manipulation of
		these values (even parsing!) can create rounding issues or
		produce values with precision beyond what is apprpriate for a
		particular currency. There are multiple approaches to dealing
		with precision. One could use integers by multiplying to a
		known-maximum precision, or use an exact decimal or rational
		representation, or simply gamble on `f64`. This library does
		not wish to make such an strong choice for the user.  2.
	 2. Any user of this library should be encouraged to create
		abstracted structures on top of the provided ones
		(e.g. creating an `Order` type specific to their application
		rather than internally passing around
		`gemini_exchange_rs::structs::Order`). Providing a reasonable
		default like `f64` for price and amount would be
		counter-productive.
