use bevy::prelude::Component;

pub enum Job {
    // rich
    Aristocrat,
    // middle
    Bureaucrat,
    // poor
    Farmer,
    Laborer,
}

pub enum Culture {
    Greek,
}

#[derive(Component)]
pub struct Pop {
    pub job: Job,
    culture: Culture,
    money: i32,
    size: i32,
}
