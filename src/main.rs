fn main() {
    use restaurant::front_of_house::hosting;
    hosting::add_to_waitlist("Ander");
    hosting::seat_at_table("Ander");
    hosting::add_to_waitlist("João");
    hosting::seat_at_table("João");

    restaurant::front_of_house::serving::take_order();
    restaurant::front_of_house::serving::serve_order();
    restaurant::front_of_house::serving::take_payment();

}