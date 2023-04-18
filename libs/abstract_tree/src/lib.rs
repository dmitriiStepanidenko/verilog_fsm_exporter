struct Module {}

macro_rules! my_module {
    ($name:ident { $($content:tt)* }) => {
        mod $name {
            $($content)*
        }
    };
}

my_module! {
    my_mod {
        pub fn my_function() {
            println!("Hello, world!");
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}