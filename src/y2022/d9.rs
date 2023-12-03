//! --- Day 9: Rope Bridge ---

pub fn part1(input: String) -> String {
    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = head;
    let mut places_visited = vec![tail];
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let direction = iter.next().unwrap();
        let distance = iter.next().unwrap().parse::<i32>().unwrap();
        for _step in 0..distance {
            let old_head = head;
            // move head
            match direction {
                "R" => head[0] += 1,
                "L" => head[0] -= 1,
                "U" => head[1] += 1,
                "D" => head[1] -= 1,
                _ => panic!("Unknown direction {}", direction),
            }
            // move tail to head´s old position if tail is not adjacent or overlapping head
            if (tail[0] - head[0]).abs() > 1 || (tail[1] - head[1]).abs() > 1 {
                tail = old_head;
                // and add this position to the places visited (if not already visited)
                if !places_visited.contains(&tail) {
                    places_visited.push(tail);
                }
            }
        }
    }
    // return the number of places visited
    places_visited.len().to_string()
}

pub fn part2(input: String) -> String {
    const KNOT_COUNT: usize = 10;
    let mut knots: [[i32; 2]; KNOT_COUNT] = [[0, 0]; KNOT_COUNT];
    let mut places_visited = vec![knots[0]];
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let direction = iter.next().unwrap();
        let distance = iter.next().unwrap().parse::<i32>().unwrap();
        for _step in 0..distance {
            //head
            // move head
            let mut old_head = knots[0];
            match direction {
                "R" => knots[0][0] += 1,
                "L" => knots[0][0] -= 1,
                "U" => knots[0][1] += 1,
                "D" => knots[0][1] -= 1,
                _ => panic!("Unknown direction {}", direction),
            }
            //body
            for i in 1..KNOT_COUNT {
                // move tail to head´s old position if tail is not adjacent or overlapping head
                if (knots[i][0] - knots[i-1][0]).abs() > 1 || (knots[i][1] - knots[i-1][1]).abs() > 1 {
                    knots[i] = old_head;
                    old_head = knots[i];
                    // tail
                    if i == KNOT_COUNT-1 {
                        // and add this position to the places visited (if not already visited)
                        if !places_visited.contains(&knots[i]) {
                            places_visited.push(knots[i]);
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }
    // return the number of places visited
    places_visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT1: &str = 
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1.to_string()), "13".to_string());
    }
    static INPUT2: &str = 
"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2.to_string()), "36".to_string());
    }
}
