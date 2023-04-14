pub mod utils {
    use fake::Fake;
    pub fn get_words(range: std::ops::Range<usize>) -> String {
        return fake::faker::lorem::en::Words(range)
            .fake::<Vec<String>>()
            .join(" ");
    }
}
