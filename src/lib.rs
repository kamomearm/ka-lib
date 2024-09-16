pub mod algebra;
pub mod data_structure;
pub mod utils;
pub mod str_algo;
pub mod compressed_mapping;
pub mod dp;
pub mod acc;

#[cfg(test)]
mod tests {
    use crate::str_algo::trie::Trie;


    #[test]
    fn it_works() {
        let _ = Trie::new();
    }
}

