use regex::Regex;

fn main() {
    let rect = parse_target_area(include_str!("input.txt")).unwrap();

    println!(
        "Heighest y reached: {}",
        heighest_possible_y(&rect).unwrap_or_default()
    );
    println!(
        "Valid initial velocities: {}",
        valid_initial_velocities(&rect)
    );
}

#[derive(Clone, Debug, PartialEq)]
struct Rect {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

impl Rect {
    fn is_within(&self, x: isize, y: isize) -> bool {
        (self.x..=self.x + self.width).contains(&x) && (self.y - self.height..=self.y).contains(&y)
    }
}

fn parse_target_area(input: &str) -> Option<Rect> {
    let regex = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();

    regex.captures(input).map(|c| {
        let from_x = c.get(1).unwrap().as_str().parse().unwrap();
        let to_x: isize = c.get(2).unwrap().as_str().parse().unwrap();
        let from_y = c.get(3).unwrap().as_str().parse().unwrap();
        let to_y: isize = c.get(4).unwrap().as_str().parse().unwrap();

        Rect {
            x: from_x,
            y: if from_y > to_y { from_y } else { to_y },
            width: to_x - from_x,
            height: (to_y - from_y).abs(),
        }
    })
}

/// ## Returns
///
/// - None if the target area is not hit with the given starting velocities.
/// - Some with the hit coordinates (x and y), as well as the heighest y reached.
fn ends_within(
    rect: &Rect,
    mut velocity_x: isize,
    mut velocity_y: isize,
) -> Option<(isize, isize, isize)> {
    let (mut x, mut y, mut heighest_y) = (0, 0, 0);

    loop {
        if rect.is_within(x, y) {
            break Some((x, y, heighest_y));
        }

        // Can we still reach it in the future?
        if x > rect.x + rect.width || y < rect.y - rect.height {
            break None;
        }

        x += velocity_x;
        y += velocity_y;

        if y > heighest_y {
            heighest_y = y;
        }

        // Simulate drag and gravity
        match velocity_x.cmp(&0) {
            std::cmp::Ordering::Greater => velocity_x -= 1,
            std::cmp::Ordering::Less => velocity_x += 1,
            _ => {}
        }
        velocity_y -= 1
    }
}

fn heighest_possible_y(rect: &Rect) -> Option<isize> {
    (0..1000)
        .flat_map(|x| (-1000..1000).map(move |y| (x, y)))
        .filter_map(|(x, y)| ends_within(rect, x, y).map(|(_, _, heighest_y)| heighest_y))
        .max()
}

fn valid_initial_velocities(rect: &Rect) -> usize {
    (0..1000)
        .flat_map(|x| (-1000..1000).map(move |y| (x, y)))
        .filter(|(x, y)| ends_within(rect, *x, *y).is_some())
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{
        ends_within, heighest_possible_y, parse_target_area, valid_initial_velocities, Rect,
    };

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_parse_target_area() {
        assert_eq!(
            Some(Rect {
                x: 20,
                y: -5,
                width: 10,
                height: 5
            }),
            parse_target_area(TEST_INPUT)
        )
    }

    #[test]
    fn test_ends_within() {
        let rect = parse_target_area(TEST_INPUT).unwrap();

        assert_eq!(Some((28, -7, 3)), ends_within(&rect, 7, 2));
        assert_eq!(Some((21, -9, 6)), ends_within(&rect, 6, 3));
        assert_eq!(Some((30, -6, 0)), ends_within(&rect, 9, 0));
        assert_eq!(None, ends_within(&rect, 17, -4));
    }

    #[test]
    fn test_heighest_possible_y() {
        let rect = parse_target_area(TEST_INPUT).unwrap();

        assert_eq!(Some(45), heighest_possible_y(&rect))
    }

    #[test]
    fn test_valid_initial_velocities() {
        let rect = parse_target_area(TEST_INPUT).unwrap();

        assert_eq!(112, valid_initial_velocities(&rect))
    }
}
