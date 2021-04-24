use std::time::Duration;

mod resolvers;

mod services;
use services::{which_service, SERVICES};

#[cfg(test)]
mod tests;

pub fn is_shortened(url: &str) -> bool {
    //! Check to see if a given url is a shortened url
    //! ## Example
    //! ```rust
    //! use urlexpand::is_shortened;
    //!
    //! fn main() {
    //!     let url = "https://bit.ly/id";
    //!     assert!(is_shortened(url));
    //! }
    //! ```
    SERVICES.iter().any(|x| url.contains(x))
}

pub fn unshorten(url: &str, timeout: Option<Duration>) -> Option<String> {
    //! UnShorten a shortened URL
    //! ## Example
    //! ```rust
    //! use std::time::Duration;
    //! use urlexpand::unshorten;
    //!
    //! fn main() {
    //!     let url = "https://bit.ly/3alqLKi";
    //!     assert!(unshorten(url, Some(Duration::from_secs(10))).is_some());   // with timeout
    //!     assert!(unshorten(url, None).is_some());    // without timeout
    //! }
    //! ```
    let service = match which_service(url) {
        Some(service) => service,
        None => return None,
    };

    match service {
        "adf.ly" |
        "fumacrom.com" |
        "intamema.com" |
        "j.gs" |
        "q.gs" => resolvers::adfly::unshort(url, timeout),
        "nowlinks.net" => resolvers::nowlinks::unshort(url, timeout),
        "rlu.ru" => resolvers::rlu::unshort(url, timeout),
        "tinyurl.com" => resolvers::tinyurl::unshort(url, timeout),
        _ => resolvers::generic::unshort(url, timeout),
    }
}
