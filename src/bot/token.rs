use regex::Regex;
use std::{
    sync::Once,
    mem::MaybeUninit,
};

#[derive(Hash)]
pub struct Token(String);

impl<'a, T> From<T> for Token 
where T:Into<String>
{
    fn from(obj: T) -> Self {
        static mut INIT: Once = Once::new();
        static mut TOKEN_PATTERN: MaybeUninit<Regex> = MaybeUninit::uninit();
        unsafe {
            INIT.call_once(|| {
                TOKEN_PATTERN = MaybeUninit::new(Regex::new(r"(\d+):(\p{Greek}+)").unwrap());
            });

            let tok = obj.into();
            if !(&*TOKEN_PATTERN.as_ptr()).is_match(tok.as_str()) {
                panic!("invalid token format");
            }
            Token(tok)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn convert_str_to_token() {
        Token::from("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11");
    }
}