//! Solutions to 2022: Advent of Code day 22
//! By Peter Fornwall

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Move { steps: i32 },
    Right,
    Left,
}

// corner pos -> 3d coord, dx-vec, dy-vec
type Faces = HashMap<(i32, i32), (glam::IVec3, glam::IVec3, glam::IVec3)>;
// 3d coord, normal -> pos
type Edges = HashMap<(glam::IVec3, glam::IVec3), (i32, i32)>;

#[derive(Debug, Default)]
struct CubeMap {
    map: Vec<Vec<u8>>,
    side_length: i32,
    x_start: i32,
    y_start: i32,

    faces: Faces,
    edges: Edges,

    line_x_min: Vec<i32>,
    line_x_max: Vec<i32>,
    column_y_min: Vec<i32>,
    column_y_max: Vec<i32>,

    instructions: Vec<Instruction>,
}

impl CubeMap {
    fn in_2d_map(&self, x: i32, y: i32) -> bool {
        y >= 0
            && y < self.map.len() as i32
            && x >= 0
            && x < self.map[y as usize].len() as i32
            && self.map[y as usize][x as usize] != b' '
    }

    fn calc_face(&mut self, x: i32, y: i32, pos3: glam::IVec3, dx: glam::IVec3, dy: glam::IVec3) {
        if !self.in_2d_map(x, y) || self.faces.contains_key(&(x, y)) {
            return;
        }
        self.faces.insert((x, y), (pos3, dx, dy));
        let normal = dy.cross(dx);
        let s = self.side_length;
        for i in 0..s {
            // Calculate mapping along the 4 edges of the face,
            // from 3d-coord & face normal => 2d-coord
            self.edges.insert((pos3 + dy * i, normal), (x, y + i));
            self.edges
                .insert((pos3 + dy * i + dx * (s - 1), normal), (x + s - 1, y + i));
            self.edges.insert((pos3 + dx * i, normal), (x + i, y));
            self.edges
                .insert((pos3 + dx * i + dy * (s - 1), normal), (x + i, y + s - 1));
        }
        // Check neighbor faces in 2d map.
        // Right neighbor, pos along dx.
        self.calc_face(x + s, y, pos3 + dx * (s - 1), normal, dy);
        // Down neighbor, pos along dy.
        self.calc_face(x, y + s, pos3 + dy * (s - 1), dx, normal);
        // Left neighbor, pos along normal ("folding inwards").
        self.calc_face(x - s, y, pos3 + normal * (s - 1), -normal, dy);
        // Up neighbor, pos along normal ("folding inwards").
        self.calc_face(x, y - s, pos3 + normal * (s - 1), dx, -normal);
    }

    fn forward(
        &self,
        is_cube: bool,
        steps: i32,
        mut x: i32,
        mut y: i32,
        mut dx: i32,
        mut dy: i32,
    ) -> (i32, i32, i32, i32) {
        for _ in 0..steps {
            let (mut tx, mut ty, mut tdx, mut tdy) = (x + dx, y + dy, dx, dy);
            if !self.in_2d_map(tx, ty) {
                if !is_cube {
                    // part 1
                    tx = if dx == 0 {
                        tx
                    } else if tx < self.line_x_min[ty as usize] {
                        self.line_x_max[ty as usize]
                    } else if tx > self.line_x_max[ty as usize] {
                        self.line_x_min[ty as usize]
                    } else {
                        tx
                    };
                    ty = if dy == 0 {
                        ty
                    } else if ty < self.column_y_min[tx as usize] {
                        self.column_y_max[tx as usize]
                    } else if ty > self.column_y_max[tx as usize] {
                        self.column_y_min[tx as usize]
                    } else {
                        ty
                    };
                } else {
                    // part 2
                    let s = self.side_length;
                    // Get 3d coords for current x,y pos (and face normal).
                    let face_corner = ((x / s) * s, (y / s) * s);
                    let (face_pos, mut dx3, mut dy3) = self.faces[&face_corner];
                    let pos3d = face_pos + dx3 * (x % s) + dy3 * (y % s);
                    let f_normal = dy3.cross(dx3);
                    // Get 2d coords for edge at pos3d with normal against our stepping direction.
                    // Either dx or dy will be zero.
                    (tx, ty) = self.edges[&(pos3d, -dy3 * dy - dx3 * dx)];

                    let face2_corner = ((tx / s) * s, (ty / s) * s);
                    (_, dx3, dy3) = self.faces[&face2_corner];
                    // "Keep" the axis that projects on the face normal
                    (tdx, tdy) = (dx3.dot(f_normal), dy3.dot(f_normal));
                }
            }
            if self.map[ty as usize][tx as usize] == b'#' {
                break;
            }
            (x, y, dx, dy) = (tx, ty, tdx, tdy);
        }
        (x, y, dx, dy)
    }

    fn follow(&self, is_cube: bool) -> i32 {
        let (mut dx, mut dy) = (1, 0);
        let (mut x, mut y) = (self.x_start, self.y_start);
        for instr in &self.instructions {
            match instr {
                Instruction::Right => (dx, dy) = (-dy, dx),
                Instruction::Left => (dx, dy) = (dy, -dx),
                Instruction::Move { steps } => {
                    (x, y, dx, dy) = self.forward(is_cube, *steps, x, y, dx, dy);
                }
            };
        }
        let test = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let cur_dir = test.iter().find_position(|x| **x == (dx, dy)).unwrap().0;
        1000 * (y + 1) + 4 * (x + 1) + cur_dir as i32
    }
}

fn parse(input: &str) -> CubeMap {
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let map: Vec<_> = map.lines().map(|line| line.as_bytes().to_vec()).collect();
    let max_width = map.iter().map(|line| line.len()).max().unwrap();
    let mut line_x_min: Vec<i32> = vec![i32::MAX; map.len()];
    let mut line_x_max: Vec<i32> = vec![i32::MIN; map.len()];
    let mut column_y_min: Vec<i32> = vec![i32::MAX; max_width];
    let mut column_y_max: Vec<i32> = vec![i32::MIN; max_width];
    let mut count = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != b' ' {
                line_x_min[y] = line_x_min[y].min(x as i32);
                line_x_max[y] = line_x_max[y].max(x as i32);
                column_y_min[x] = column_y_min[x].min(y as i32);
                column_y_max[x] = column_y_max[x].max(y as i32);
                count += 1;
            }
        }
    }
    let side_length = ((count / 6) as f32).sqrt() as i32;

    // Assumes empty space on first line
    let x_start = map[0]
        .iter()
        .enumerate()
        .find(|(_, c)| **c == b'.')
        .unwrap()
        .0 as i32;

    let re = Regex::new(r"\d+|R|L").unwrap();
    let instructions: Vec<_> = re
        .find_iter(instructions.trim())
        .map(|m| match m.as_str() {
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            x => Instruction::Move {
                steps: x.parse::<i32>().unwrap(),
            },
        })
        .collect();

    let mut cube_map = CubeMap {
        map,
        side_length,
        x_start,
        y_start: 0,
        faces: Faces::default(),
        edges: Edges::default(),
        line_x_min,
        line_x_max,
        column_y_min,
        column_y_max,
        instructions,
    };
    cube_map.calc_face(
        cube_map.line_x_min[0],
        0,
        glam::IVec3::ZERO,
        glam::IVec3::X,
        glam::IVec3::Y,
    );
    cube_map
}

fn part1(input: &str) -> i32 {
    let cube_map = parse(input);
    cube_map.follow(false)
}

fn part2(input: &str) -> i32 {
    let cube_map = parse(input);
    cube_map.follow(true)
}

fn main() {
    let input = include_str!("../../../inputs/day22_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 123046);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 195032);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day22_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 6032);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 5031);
}
