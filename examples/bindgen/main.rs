fn main() {
    println!("The value is {}!", simple_bindgen::SIMPLE_VALUE);
}

#[cfg(test)]
mod test {
    #[test]
    fn do_the_test() {
        assert_eq!(42, simple_bindgen::SIMPLE_VALUE);
        #[cfg(target_os = "macos")]
        assert_ne!(-1, simple_bindgen::SIMPLE_IS_MACOS);
    }
}
