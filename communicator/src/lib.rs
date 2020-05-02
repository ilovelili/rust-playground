/*
* the line mod client; means this:
* mod client {
*     // contents of client.rs
* }
*/
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    // use statement paths are relative to the crate root by default
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }
}
