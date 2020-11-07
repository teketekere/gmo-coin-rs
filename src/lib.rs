//! This is a first-step crate.

#![crate_name = "rust_docker_sample"]

/// Returns an string which has same value with an arugment msg.
///
/// # Arguments
/// * `msg` - A string slice
///
/// # Examples
///
/// ```
/// use rust_docker_sample::echo;
/// let msg: &str = "msg...";
/// let echo_msg = echo(msg);
///```
///
pub fn echo(msg: &str) -> String {
    return String::from(msg);
}

#[cfg(test)]
mod tests {
    use crate::echo;

    #[test]
    fn test_echo() {
        let msg: &str = "yo man";
        let echo_msg = echo(msg);
        assert!(echo_msg == "yo man");
    }
}
