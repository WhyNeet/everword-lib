#[macro_export]
macro_rules! none_return {
    ($option:expr) => {
        if Option::is_none(option) {
            return None;
        } else {
            option.unwrap()
        }
    };
}
