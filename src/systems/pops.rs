use bevy::{prelude::*, utils::HashMap};

use crate::components::{
    economy::TradeGoods,
    pops::{Job, Pop},
};

pub fn place_orders(mut query: Query<&Pop>) {
    let mut orders = HashMap::new();

    for (pop) in &mut query {
        let needs = match pop.job {
            Job::Bureaucrat => (TradeGoods::Pottery, 5 * pop.size),
            _ => 5,
        };

        orders.insert()
    }
}
