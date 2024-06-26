pub mod shirt {
    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    pub fn run() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }
}

/// find the shoes in the given size
///
/// # Examples
///
/// ```
/// use rust::rb::_13::shoe:: { Shoe, shoes_in_size };
///
/// let shoes = vec![
///     Shoe {
///         size: 10,
///         style: String::from("sneaker"),
///     },
///     Shoe {
///         size: 13,
///         style: String::from("sandal"),
///     },
///     Shoe {
///         size: 10,
///         style: String::from("boot"),
///     },
/// ];
///
/// let in_my_size = shoes_in_size(shoes, 10);
///
/// assert_eq!(
///     in_my_size,
///     vec![
///         Shoe {
///             size: 10,
///             style: String::from("sneaker")
///         },
///         Shoe {
///             size: 10,
///             style: String::from("boot")
///         },
///     ]
/// );
/// ```
pub mod shoe {
    #[derive(PartialEq, Debug)]
    pub struct Shoe {
        pub size: u32,
        pub style: String,
    }

    pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
}
