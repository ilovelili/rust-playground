pub mod front_of_house {
    pub fn add_to_waitlist() {}
}

pub fn eat_at_restaurant() {
    front_of_house::add_to_waitlist();
}
