#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0;
        let mut num_blue = 0;

        for colour in &self.shirts {
            match colour {
                ShirtColour::Blue => num_blue += 1,
                ShirtColour::Red => num_red += 1,
            }
        }
        if num_red > num_blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };
    let user_pref1 = Some(ShirtColour::Red);
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
