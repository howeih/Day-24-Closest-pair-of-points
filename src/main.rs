use std::cmp::Ordering;
use std::f64;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn dist(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn sort_by_x(p1: &Point, p2: &Point) -> Ordering {
    let a = p1.x;
    let b = p2.x;
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn sort_by_y(p1: &Point, p2: &Point) -> Ordering {
    let a = p1.y;
    let b = p2.y;
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn brutal_force(points: &[Point]) -> f64 {
    let mut closet_dist = f64::MAX;
    for p1 in 0..points.len() - 1 {
        for p2 in p1 + 1..points.len() {
            let d = dist(&points[p1], &points[p2]);
            if d < closet_dist {
                closet_dist = d;
            }
        }
    }
    closet_dist
}

fn search_mid(points: Vec<Point>, min_dist: f64) -> f64 {
    let mut result = min_dist;
    let point_len = points.len() as i32;
    for pi in 0..point_len - 2 {
        for yi in pi + 1..point_len - 1 {
            if points[yi as usize].y - points[pi as usize].y > min_dist {
                break;
            }
            let dist = dist(&points[yi as usize], &points[pi as usize]);
            if dist  < min_dist{
                result = dist;
            }
        }
    }
    result
}

fn search(points: &Vec<Point>, left: usize, length: usize) -> f64 {
    let min_dist;
    if length <= 3 {
        return brutal_force(&points[left..left + length]);
    }
    let mid = left + length / 2;
    let left_dist = search(points, left, length / 2);
    let right_dist = search(points, mid, length / 2);
    if left_dist < right_dist {
        min_dist = left_dist;
    } else {
        min_dist = right_dist;
    }
    let strip: Vec<Point> = points
        .iter()
        .filter_map(|p| {
            if (p.x - points[mid].x).abs() < min_dist {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect();
    search_mid(strip, min_dist)
}

fn closet_pair(points: &mut Vec<Point>) -> f64 {
    points.sort_by(sort_by_x);
    search(&points, 0, points.len() - 1)
}

fn main() {
    let mut points = vec![
        Point { x: 0., y: 2. },
        Point { x: 1., y: 6. },
        Point { x: 1., y: 4. },
        Point { x: 2., y: 7. },
        Point { x: 5., y: 5. },
        Point { x: 7., y: 1. },
        Point { x: 8., y: 9. },
        Point { x: 8., y: 5. },
        Point { x: 9., y: 9. },
        Point { x: 9., y: 6. },
    ];
    points.sort_by(sort_by_y);
    assert_eq!(1.4142135623730951, closet_pair(&mut points));
}
