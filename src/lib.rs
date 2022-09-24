use arrayvec::ArrayVec;
use std::fmt::Write;
use wasm_bindgen::prelude::*;

fn get_permutations() -> ArrayVec<ArrayVec<u8, 6>, 720> {
    let mut result: ArrayVec<ArrayVec<u8, 6>, 720> = ArrayVec::new();

    fn helper(
        result: &mut ArrayVec<ArrayVec<u8, 6>, 720>,
        temp: ArrayVec<u8, 6>,
        rest: ArrayVec<u8, 6>,
        depth: u8,
    ) {
        if depth == 6 {
            result.push(temp);
            return;
        } else {
            for i in 0..(6 - depth) {
                let mut removed = rest.clone();
                removed.remove(i as usize);
                let mut inserted = temp.clone();
                inserted.push(rest[i as usize]);
                helper(result, inserted, removed, depth + 1);
            }
        }
    }

    helper(
        &mut result,
        ArrayVec::new(),
        ArrayVec::from([0, 1, 2, 3, 4, 5]),
        0,
    );

    result
}

fn get_combinations() -> ArrayVec<ArrayVec<u8, 5>, 1024> {
    let mut result: ArrayVec<ArrayVec<u8, 5>, 1024> = ArrayVec::new();

    fn helper(
        result: &mut ArrayVec<ArrayVec<u8, 5>, 1024>,
        temp: ArrayVec<u8, 5>,
        rest: &ArrayVec<u8, 4>,
        depth: u8,
    ) {
        if depth == 5 {
            result.push(temp);
            return;
        } else {
            for i in 0..4 {
                let mut inserted = temp.clone();
                inserted.push(rest[i as usize]);
                helper(result, inserted, rest, depth + 1);
            }
        }
    }

    helper(
        &mut result,
        ArrayVec::new(),
        &ArrayVec::from([0, 1, 2, 3]),
        0,
    );

    result
}

fn calc(x1: u32, x2: u8, y: u8) -> u32 {
    match y {
        0 => x1 + x2 as u32,
        1 => x1 - x2 as u32,
        2 => x1 * x2 as u32,
        _ => x1 / x2 as u32,
    }
}

fn op_2_str(x: u8) -> &'static str {
    match x {
        0 => "+",
        1 => "-",
        2 => "*",
        _ => "/",
    }
}

#[wasm_bindgen]
pub struct Solver {
    permutations: ArrayVec<ArrayVec<u8, 6>, 720>,
    combinations: ArrayVec<ArrayVec<u8, 5>, 1024>,
    equations: Vec<String>,
}

#[wasm_bindgen]
impl Solver {
    pub fn new() -> Solver {
        Solver {
            permutations: get_permutations(),
            combinations: get_combinations(),
            equations: Vec::new(),
        }
    }

    pub fn solve_equations(&mut self, nums: &[u8], target: u32) -> usize {
        let mut results: Vec<String> = Vec::new();

        for p in &self.permutations {
            let sorted_nums: ArrayVec<u8, 6> = p.into_iter().map(|x| nums[*x as usize]).collect();

            for c in &self.combinations {
                let mut result = sorted_nums[0] as u32;

                for i in 0..5 {
                    if result < sorted_nums[i + 1] as u32
                        || (c[i] == 3 && result % sorted_nums[i + 1] as u32 != 0)
                    {
                        break;
                    }
                    result = calc(result, sorted_nums[i + 1], c[i]);

                    if result == target {
                        let mut result_str = sorted_nums[0].to_string();

                        for j in 0..(i + 1) {
                            write!(result_str, " {} {}", op_2_str(c[j]), sorted_nums[j + 1])
                                .unwrap();
                        }

                        results.push(result_str);
                    }
                }
            }
        }

        let len = results.len();
        self.equations = results;

        len
    }

    pub fn get_equations(&self, i: usize) -> String {
        self.equations[i].clone()
    }
}

#[test]
fn test() {
    let mut x = Solver::new();
    let y: [u8; 6] = [1, 2, 3, 4, 5, 6];
    x.solve_equations(&y, 21);
    let z = 0;
}
