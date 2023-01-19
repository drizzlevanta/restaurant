mod front_of_house;

mod back_of_house;

pub fn eat_at_restaurant() {
    //absolutely path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Italian");

    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}
