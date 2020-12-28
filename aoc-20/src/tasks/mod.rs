mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;

pub fn dispatch(input_root: &str, day: &Day, part: &Part) {
  match day {
    Day::Day01 => match part {
      Part::First => day_01::solve_part_1(input_root),
      Part::Second => day_01::solve_part_2(input_root),
    },
    Day::Day02 => match part {
      Part::First => day_02::solve_part_1(input_root),
      Part::Second => day_02::solve_part_2(input_root),
    },
    Day::Day03 => match part {
      Part::First => day_03::solve_part_1(input_root),
      Part::Second => day_03::solve_part_2(input_root),
    },
    Day::Day04 => match part {
      Part::First => day_04::solve_part_1(input_root),
      Part::Second => day_04::solve_part_2(input_root),
    },
    Day::Day05 => match part {
      Part::First => day_05::solve_part_1(input_root),
      Part::Second => day_05::solve_part_2(input_root),
    },
    Day::Day06 => match part {
      Part::First => day_06::solve_part_1(input_root),
      Part::Second => day_06::solve_part_2(input_root),
    },
    Day::Day07 => match part {
      Part::First => day_07::solve_part_1(input_root),
      Part::Second => day_07::solve_part_2(input_root),
    },
    Day::Day08 => match part {
      Part::First => day_08::solve_part_1(input_root),
      Part::Second => day_08::solve_part_2(input_root),
    },
    Day::Day09 => match part {
      Part::First => day_09::solve_part_1(input_root),
      Part::Second => day_09::solve_part_2(input_root),
    },
    Day::Day10 => match part {
      Part::First => day_10::solve_part_1(input_root),
      Part::Second => day_10::solve_part_2(input_root),
    },
    Day::Day11 => match part {
      Part::First => day_11::solve_part_1(input_root),
      Part::Second => day_11::solve_part_2(input_root),
    },
  }
}

pub enum Day {
  Day01,
  Day02,
  Day03,
  Day04,
  Day05,
  Day06,
  Day07,
  Day08,
  Day09,
  Day10,
  Day11,
}

pub enum Part {
  First,
  Second,
}
