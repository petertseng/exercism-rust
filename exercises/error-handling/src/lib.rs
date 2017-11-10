#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum IceCream {
    Chocolate,
    Vanilla,
    Strawberry,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    AmbiguousFlavour,
    NoSuchFlavour,
}

type Result<T> = std::result::Result<T, Error>;

impl IceCream {
    pub fn get_flavor(s: &str) -> Result<IceCream> {
        let choices = [
            (IceCream::Chocolate, "chocolate"),
            (IceCream::Vanilla, "vanilla"),
            (IceCream::Vanilla, "plain"),
            (IceCream::Strawberry, "strawberry"),
        ];
        let lower_s = s.to_lowercase();
        let matching_choices: Vec<_> = choices.iter().filter(|&&(_, s)| s.starts_with(&lower_s)).collect();
        if matching_choices.len() == 0 {
            Err(Error::NoSuchFlavour)
        } else if matching_choices.len() > 1 {
            Err(Error::AmbiguousFlavour)
        } else {
            Ok(matching_choices[0].0)
        }
    }
}

#[derive(Debug)]
pub struct IceCreamTruck {
    // These fields are only pub, because tests are not in the same file
    // and we need to verify inventory after actions.
    pub chocolate: u32,
    pub vanilla: u32,
    pub strawberry: u32,
}

impl IceCreamTruck {
    pub fn new(chocolate: u32, vanilla: u32, strawberry: u32) -> Self {
        IceCreamTruck { chocolate: chocolate,
                        vanilla: vanilla,
                        strawberry: strawberry }
    }

    /// Buy ice cream and update inventory.
    /// Return quantity purchased or indication that you could not purchase.
    pub fn buy(&mut self, want: u32, flavour: IceCream) -> Option<u32> {
        let (r, new) = {
            let (have, set): (u32, Box<Fn(u32) -> IceCreamTruck>) = match flavour {
                IceCream::Chocolate  => (self.chocolate , Box::new(|v| IceCreamTruck { chocolate : v, .. *self })),
                IceCream::Vanilla    => (self.vanilla   , Box::new(|v| IceCreamTruck { vanilla   : v, .. *self })),
                IceCream::Strawberry => (self.strawberry, Box::new(|v| IceCreamTruck { strawberry: v, .. *self })),
            };

            if have == 0 {
                (None, set(0))
            } else if have > want {
                (Some(want), set(have - want))
            } else {
                (Some(have), set(0))
            }
        };
        *self = new;
        r
    }

    /// Receives a Vector of tuples with string name of flavor and quantity.
    /// Converts string name into flavor and buy ice cream.
    ///
    /// Returns a vector of quantity_ordered if flavor is valid and
    /// ice cream inventory exists or None.
    pub fn process_order<'a, T: AsRef<[(&'a str, u32)]>>(&mut self, order: T) -> Vec<Option<u32>> {
        order.as_ref().into_iter().map(|&(s, n)| match IceCream::get_flavor(s) {
            Ok(f)  => self.buy(n, f),
            Err(_) => None,
        }).collect()
    }
}
