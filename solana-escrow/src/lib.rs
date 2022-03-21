pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
