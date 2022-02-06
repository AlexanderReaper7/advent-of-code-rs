//! --- Day 5: Hydrothermal Venture ---

pub fn part1(input: String) -> String {
    // lines in ((x1, y1),(x2, y2)) format
    let lines: Vec<((u32, u32), (u32, u32))> = 
    input.lines()
    .map(|line| line.split(" -> ").collect::<Vec<&str>>())
    .map(|sides| 
        sides.into_iter()
        .map(|side| side.split(",").collect::<Vec<&str>>())
        .map(|numbers| (numbers[0].parse::<u32>().unwrap(), numbers[1].parse::<u32>().unwrap())).collect::<Vec<(u32, u32)>>()
    ).map(|points| (points[0], points[1]))
    .collect();

    let max_x = lines.iter().map(|line| line.0.0.max(line.1.0)).max().unwrap();
    let max_y = lines.iter().map(|line| line.0.1.max(line.1.1)).max().unwrap();

    let horizontal_lines: Vec<((u32, u32), (u32, u32))> = lines.iter().filter(|line| line.0.1 == line.1.1).map(|l| l.clone()).collect::<Vec<((u32, u32), (u32, u32))>>();
    let vertical_lines: Vec<((u32, u32), (u32, u32))> = lines.iter().filter(|line| line.0.0 == line.1.0).map(|l| l.clone()).collect::<Vec<((u32, u32), (u32, u32))>>();

    let mut table: Vec<Vec<u32>> = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];
    for line in &horizontal_lines {
        for x in line.0.0.min(line.1.0)..=line.0.0.max(line.1.0) {
            table[line.0.1 as usize][x as usize] += 1;
        }
    }
    for line in &vertical_lines {
        for y in line.0.1.min(line.1.1)..=line.0.1.max(line.1.1) {
            table[y as usize][line.0.0 as usize] += 1;
        }
    }

    let intersections = table.iter().map(|row| row.iter().filter(|&x| *x > 1).count()).sum::<usize>();
    intersections.to_string()
}

pub fn part2(input: String) -> String {
    // lines in ((x1, y1),(x2, y2)) format
    let lines: Vec<((u32, u32), (u32, u32))> = 
    input.lines()
    .map(|line| line.split(" -> ").collect::<Vec<&str>>())
    .map(|sides| 
        sides.into_iter()
        .map(|side| side.split(",").collect::<Vec<&str>>())
        .map(|numbers| (numbers[0].parse::<u32>().unwrap(), numbers[1].parse::<u32>().unwrap())).collect::<Vec<(u32, u32)>>()
    ).map(|points| (points[0], points[1]))
    .collect();

    let max_x = lines.iter().map(|line| line.0.0.max(line.1.0)).max().unwrap();
    let max_y = lines.iter().map(|line| line.0.1.max(line.1.1)).max().unwrap();

    let horizontal_lines: Vec<((u32, u32), (u32, u32))> = lines.iter().filter(|line| line.0.1 == line.1.1).map(|l| l.clone()).collect::<Vec<((u32, u32), (u32, u32))>>();
    let vertical_lines: Vec<((u32, u32), (u32, u32))> = lines.iter().filter(|line| line.0.0 == line.1.0).map(|l| l.clone()).collect::<Vec<((u32, u32), (u32, u32))>>();
    let diagonal_lines: Vec<((u32, u32), (u32, u32))> = lines.iter().filter(|line| (line.0.0 as i32 - line.1.0 as i32).abs() == (line.0.1 as i32 - line.1.1 as i32).abs()).map(|l| l.clone()).collect::<Vec<((u32, u32), (u32, u32))>>();

    let mut table: Vec<Vec<u32>> = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];
    for line in &horizontal_lines {
        for x in line.0.0.min(line.1.0)..=line.0.0.max(line.1.0) {
            table[line.0.1 as usize][x as usize] += 1;
        }
    }
    for line in &vertical_lines {
        for y in line.0.1.min(line.1.1)..=line.0.1.max(line.1.1) {
            table[y as usize][line.0.0 as usize] += 1;
        }
    }
    for line in &diagonal_lines {
        let min_x = line.0.0.min(line.1.0);
        let length = line.0.0.max(line.1.0) - min_x;
        let min_y = line.0.1.min(line.1.1);
        for offset in 0..=length {
            table[(min_y + offset) as usize][(min_x + offset) as usize] += 1;
        }
    }

    let intersections = table.iter().map(|row| row.iter().filter(|&x| *x > 1).count()).sum::<usize>();
    intersections.to_string()
}
