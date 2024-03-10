use std::{collections::HashMap, hash::Hash};

struct Product {
    price: i32,
    name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coin(i32);

impl Coin {
    fn new(value: i32) -> Self {
        if value == 1 || value == 5 || value == 10 || value == 25 {
            Self(value)
        } else {
            panic!("Invalid coin value")
        }
    }
}

struct VendingMachine {
    products: Vec<Product>,
    vault: HashMap<Coin, i32>,
}

struct Change {
    coins: Vec<Coin>,
    target: i32,
}

fn make_change(vault: &mut HashMap<Coin, i32>, candidates: &mut Vec<Change>) {
    let coins: Vec<Coin> = vault.keys().cloned().collect();
    //    let mut candidates: Vec<Change> = Vec::new();
    //    for coin in vault.keys().cloned().collect() {
    //        let coin: Coin = coin;
    //        if target - coin.0 > 0 {
    //            candidates.push(Change {
    //                coins: vec![coin],
    //                target,
    //            })
    //        }
    //    }

    //    for coin in coins {
    //        let candidate = target - coin.0;
    //        if candidate >= 0 {
    //            let a = vault.get(&coin).unwrap();
    //            let new_amount = *a - 1;
    //            if new_amount > 0 {
    //                vault.insert(coin, *a - 1);
    //            } else {
    //                vault.remove(&coin);
    //            }
    //            candidates.push(vec![candidate])
    //        }
    //    }
    //    Change {
    //        coins: vec![],
    //        target,
    //    }
}
