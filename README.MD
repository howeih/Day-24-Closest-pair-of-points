#Day 24: Closest pair of points
<br>
https://en.wikipedia.org/wiki/Closest_pair_of_points_problem
<br>
Given a set of 2D points, the mission is to find the closest pair.
<br>
Naive strategy would be to examine all the pairs and choose the closest one. But naive is also expensive, leading to O(n²) time complexity.
<br>
Alternatively, we can use divide and conquer to achieve O(n.log(n)) time. Sort the points, make a split by median and find the closest pair on left and on right of median. Finally, merge points on left with points on right and examine pairs around median.
<br>
What’s particularly interesting about the algorithm? It looks like O(n²) algorithm! If you take a closer look at the inner cycle when searching the middle band, it might make up to n²/4 steps. However, it can be proven that the cycle makes 6 steps at worst.

```
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
```
