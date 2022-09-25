use bevy::prelude::*;

pub enum TradeGoods {
    // Raw materials
    Gold,
    Copper,
    Ivory,
    // Finished goods
    Pottery,
    Wine,
    // Military goods
    Artillery,
}

#[derive(Component)]
pub struct MarketOrder {
    buy: bool,
    good: TradeGoods,
}
