use std::collections::HashMap;
use std::fs;

/*
The plan is to represent the diagram as a hashmap with {coordinate: number of points crossing it}
 */

fn main() {
    part1("data/example01.txt");
    part2("data/example01.txt");

    part1("data/real01.txt");
    part2("data/real01.txt");
}

pub fn part2(filename: &str) {
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let lines_string: Vec<String> = data.lines().map(|s| s.to_owned()).collect();
    let mut points_map: HashMap<Point, i32> = HashMap::new();
    let mut lines: Vec<Line> = vec![];
    for line_s in lines_string {
        lines.push(parse_line(line_s));
    }

    for line in lines {
        let line_points = line.find_points_between();
        for point in line_points {
            *points_map.entry(point).or_insert(0) += 1;
        }
    }
    let mut times_above_1: i32 = 0;
    for (_k, v) in points_map.iter() {
        if v > &1 {
            times_above_1 += 1;
        }
    }
    println!("part 2: {:?}", times_above_1);
}

pub fn part1(filename: &str) {
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let lines_string: Vec<String> = data.lines().map(|s| s.to_owned()).collect();
    let mut points_map: HashMap<Point, i32> = HashMap::new();
    let mut lines: Vec<Line> = vec![];
    for line_s in lines_string {
        lines.push(parse_line(line_s));
    }

    for line in lines {
        if let Some(line_points) = line.find_points_between_vertical_or_horizontal() {
            for point in line_points {
                *points_map.entry(point).or_insert(0) += 1;
            }
        } else {
            continue;
        }
    }
    let mut times_above_1: i32 = 0;
    for (_k, v) in points_map.into_iter() {
        if v > 1 {
            times_above_1 += 1;
        }
    }
    println!("part 1: {:?}", times_above_1);
}

pub fn parse_line(s: String) -> Line {
    // s = "0,9 -> 5,9"
    let points_vec: Vec<String> = s.split(" -> ").map(|x| x.to_string()).collect();
    let (x, y) =
        points_vec[0].split_at(points_vec[0].find(",").expect("Could not find split char"));
    let point_start = Point(
        x.parse::<i32>().expect("parsing error"),
        y[1..].parse::<i32>().expect("parsing error"),
    );

    let (v, w) =
        points_vec[1].split_at(points_vec[1].find(",").expect("Could not find split char"));
    let point_end = Point(
        v.parse::<i32>().expect("parsing error"),
        w[1..].parse::<i32>().expect("parsing error"),
    );
    Line {
        start: point_start,
        end: point_end,
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Point(i32, i32);

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn is_vertical(&self) -> bool {
        if (self.start.0 == self.end.0) || (self.start.1 == self.end.1) {
            return true;
        }
        false
    }

    pub fn find_points_between(&self) -> Vec<Point> {
        let delta_x = self.start.0 - self.end.0;
        let delta_y = self.start.1 - self.end.1;
        let mut points: Vec<Point> = vec![];

        if self.is_vertical() {
            points.push(self.start.clone());
            points.push(self.end.clone());
        }
        if (delta_x != 0) && (delta_y == 0) {
            if delta_x > 0 {
                // end -> start
                for i in (self.end.0 + 1)..self.start.0 {
                    points.push(Point(i, self.end.1)); // does not matter as long it is y
                }
            } else {
                // start -> end
                for i in (self.start.0 + 1)..self.end.0 {
                    points.push(Point(i, self.end.1));
                }
            }
        } else if (delta_x == 0) && (delta_y != 0) {
            // delta_x = 0
            if delta_y > 0 {
                // end -> start
                for i in (self.end.1 + 1)..self.start.1 {
                    points.push(Point(self.end.0, i));
                }
            } else {
                // start -> end
                for i in (self.start.1 + 1)..self.end.1 {
                    points.push(Point(self.end.0, i));
                }
            }
        } else {
            if (self.start.0 > self.end.0) && (self.start.1 < self.end.1) {
                // decrease x increase y
                let x_range = ((self.end.0)..(self.start.0 + 1)).rev();
                let y_range = (self.start.1)..(self.end.1 + 1);
                for (i, j) in x_range.zip(y_range) {
                    let line = Line::new(Point(i, j), Point(self.end.0, self.end.1));
                    if line.is_diagonal() {
                        points.push(Point(i, j));
                    }
                }
            } else if (self.start.0 < self.end.0) && (self.start.1 > self.end.1) {
                // increase x decrease y
                let x_range = (self.start.0)..(self.end.0 + 1);
                let y_range = ((self.end.1)..(self.start.1 + 1)).rev();
                for (i, j) in x_range.zip(y_range) {
                    let line = Line::new(Point(i, j), Point(self.end.0, self.end.1));
                    if line.is_diagonal() {
                        points.push(Point(i, j));
                    }
                }
            } else if (self.start.0 > self.end.0) && (self.start.1 > self.end.1) {
                // decrease x decrease y
                let x_range = ((self.end.0)..(self.start.0 + 1)).rev();
                let y_range = ((self.end.1)..(self.start.1 + 1)).rev();
                for (i, j) in x_range.zip(y_range) {
                    let line = Line::new(Point(i, j), Point(self.end.0, self.end.1));
                    if line.is_diagonal() {
                        points.push(Point(i, j));
                    }
                }
            } else {
                // increase x increase y
                let x_range = (self.start.0)..(self.end.0 + 1);
                let y_range = (self.start.1)..(self.end.1 + 1);
                for (i, j) in x_range.zip(y_range) {
                    let line = Line::new(Point(i, j), Point(self.end.0, self.end.1));
                    if line.is_diagonal() {
                        points.push(Point(i, j));
                    }
                }
            }
        }
        points
    }

    pub fn is_diagonal(&self) -> bool {
        if (self.start.0 - self.end.0).abs() == (self.start.1 - self.end.1).abs() {
            return true;
        }
        return false;
    }

    pub fn find_points_between_vertical_or_horizontal(&self) -> Option<Vec<Point>> {
        if !self.is_vertical() {
            return None;
        }

        let delta_x = self.start.0 - self.end.0;
        let delta_y = self.start.1 - self.end.1;
        let mut points: Vec<Point> = vec![];

        if delta_x != 0 {
            if delta_x > 0 {
                // end -> start
                for i in (self.end.0 + 1)..self.start.0 {
                    points.push(Point(i, self.end.1)); // does not matter as long it is y
                }
            } else {
                // start -> end
                for i in (self.start.0 + 1)..self.end.0 {
                    points.push(Point(i, self.end.1));
                }
            }
        } else {
            // delta_x = 0
            if delta_y > 0 {
                // end -> start
                for i in (self.end.1 + 1)..self.start.1 {
                    points.push(Point(self.end.0, i));
                }
            } else {
                // start -> end
                for i in (self.start.1 + 1)..self.end.1 {
                    points.push(Point(self.end.0, i));
                }
            }
        }
        points.push(self.start.clone());
        points.push(self.end.clone());
        Some(points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_diagonal() {
        let line1 = Line::new(Point(9, 7), Point(7, 9));
        let line2 = Line::new(Point(1, 1), Point(3, 3));
        assert!(line1.is_diagonal());
        assert!(line2.is_diagonal());
    }
}
