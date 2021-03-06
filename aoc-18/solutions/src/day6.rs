use crate::input_reader;

type Point = (isize, isize);

#[derive(Debug)]
struct BoundingBox {
    top: isize,
    right: isize,
    bottom: isize,
    left: isize,
}

impl BoundingBox {
    fn determine_box(points: &[Point]) -> BoundingBox {
        let top = points.iter().min_by_key(|p| p.0).unwrap().0;
        let left = points.iter().min_by_key(|p| p.1).unwrap().1;
        let bottom = points.iter().max_by_key(|p| p.1).unwrap().1;
        let right = points.iter().max_by_key(|p| p.0).unwrap().0;

        BoundingBox {
            top,
            right,
            bottom,
            left,
        }
    }

    fn contains(&self, p: &Point) -> bool {
        let (x, y) = *p;
        x > self.left && x < self.right && y > self.top && y < self.bottom
    }

    fn iter(&self) -> BoxIter {
        BoxIter {
            _box: self,
            curr_pos: Some((self.left - 1, self.top)),
        }
    }
}

struct BoxIter<'a> {
    _box: &'a BoundingBox,
    curr_pos: Option<Point>,
}

impl<'a> Iterator for BoxIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_pos {
            None => None,
            Some(p) => {
                self.curr_pos = match p {
                    (_, y) if y > self._box.bottom => None,
                    (x, y) if x > self._box.right => Some((self._box.left, y + 1)),
                    (x, y) => Some((x + 1, y)),
                };
                self.curr_pos
            }
        }
    }
}

enum Closest {
    One(Point),
    Several,
}

fn closest_to(p: Point, cords: &[Point]) -> Closest {
    use self::Closest::*;

    let distances = cords
        .iter()
        .map(|&cp| (cp, manhattan(p, cp)))
        .collect::<Vec<_>>();
    let (min_p, min_dist) = distances.iter().min_by_key(|(_, dist)| dist).unwrap();

    match distances
        .iter()
        .filter(|(_, dist)| *dist == *min_dist)
        .count()
    {
        x if x == 1 => One(*min_p),
        _ => Several,
    }
}

fn dist_sum(p: Point, cords: &[Point]) -> isize {
    cords.iter().map(|&cp| manhattan(p, cp)).sum()
}

fn manhattan(p1: Point, p2: Point) -> isize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    (x1 - x2).abs() + (y1 - y2).abs()
}

fn point(input: &str) -> Point {
    let arr = input.split(',').collect::<Vec<_>>();
    (
        arr[0].trim().parse().unwrap(),
        arr[1].trim().parse().unwrap(),
    )
}

fn part1(input_location: &str) -> (Point, u32) {
    use self::Closest::*;
    use std::collections::HashMap;

    let data = input_reader::read_all_lines(input_location);
    let points = data.into_iter().map(|s| point(&s)).collect::<Vec<_>>(); //make points from user input
    let bound_box = BoundingBox::determine_box(&points); //determine edges of box, finite area must be within
    let mut counter: HashMap<Point, u32> = HashMap::new(); //map from user point to number of points that are exclusively closest to it

    bound_box
        .iter()
        .filter_map(|p| match closest_to(p, &points) {
            Several => None,
            One(x) => Some(x),
        })
        .for_each(|p| *counter.entry(p).or_insert(0) += 1);

    counter
        .into_iter()
        .filter(|(k, _)| bound_box.contains(k))
        .max_by_key(|(_, v)| *v)
        .unwrap()
}

fn part2(input_location: &str) -> usize {
    let data = input_reader::read_all_lines(input_location);
    let points = data.into_iter().map(|s| point(&s)).collect::<Vec<_>>(); //make points from user input
    let bound_box = BoundingBox::determine_box(&points);
    let threshold = 10000; //Defined in task

    bound_box
        .iter()
        .map(|p| dist_sum(p, &points))
        .filter(|&d| d < threshold)
        .count()
}

pub fn day6() {
    let input = String::from("day6");
    println!("***Day Six***");
    println!("\tReading from {}", input);
    println!("\t**Part One**");
    println!("\t\tLargest non-infinite area: {:?}", part1(&input));
    println!("\t**Part Two**");
    println!("\t\tRegion of close locations: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::day6::*;
    use crate::input_reader;

    #[test]
    fn dist_sum_test() {
        let data = input_reader::read_all_lines(&String::from("day6_test"));
        let points = data.into_iter().map(|s| point(&s)).collect::<Vec<_>>();

        let d = dist_sum((4, 3), &points);
        assert_eq!(d, 30);
    }

    #[test]
    fn part2_test() {
        let data = input_reader::read_all_lines(&String::from("day6_test"));
        let points = data.into_iter().map(|s| point(&s)).collect::<Vec<_>>();
        let bound_box = BoundingBox::determine_box(&points);
        let test_threshold = 32;
        let count = bound_box
            .iter()
            .map(|p| dist_sum(p, &points))
            .filter(|&d| d < test_threshold)
            .count();

        assert_eq!(count, 16);
    }
}
