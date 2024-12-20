use std::time::Instant;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;

fn main() {
    let start = Instant::now();
    // day_01::part_1();
    // day_01::part_2();

    // day_02::part_1("./inputs/day_02.txt");
    // day_02::part_2("./inputs/day_02.txt");

    // day_03::part_1("./inputs/day_03.txt");
    // day_03::part_2("./inputs/day_03.txt");

    // day_04::part_1("./inputs/day_04.txt");
    // day_04::part_2("./inputs/day_04.txt");

    // day_05::part_1("./inputs/day_05.txt");
    // day_05::part_2("./inputs/day_05.txt");

    // day_06::part_1("./inputs/day_06.txt");
    // day_06::part_2("./inputs/day_06.txt");

    // day_07::part_1("./inputs/day_07.txt");
    // day_07::part_2("./inputs/day_07.txt");

    // day_08::part_1("./inputs/day_08.txt");
    // day_08::part_2("./inputs/day_08.txt");

    // day_09::part_1("./inputs/day_09.txt");
    // day_09::part_2("./inputs/day_09.txt");

    // day_10::part_1("./inputs/day_10.txt");
    // day_10::part_2("./inputs/day_10.txt");

    // day_11::both_parts("./inputs/day_11.txt", 25);
    day_11::both_parts("./inputs/day_11.txt", 200);

    println!("Spent {:?}", start.elapsed());
}
