use std::collections::{HashSet, VecDeque};

use good_lp::{IntoAffineExpression, ProblemVariables};
use good_lp::{Solution, SolverModel, default_solver, variable};
use regex::Regex;

use crate::util;

#[derive(Debug)]
struct Problem {
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

fn parse_goal(s: &str) -> Vec<bool> {
    s.bytes()
        .map(|b| match b {
            b'.' => false,
            b'#' => true,
            _ => panic!("unexpected input: {}", b as char),
        })
        .collect()
}

fn parse_buttons(s: &str) -> Vec<Vec<usize>> {
    s.split(' ')
        .map(|s| {
            util::parse_ints(s)
                .expect("failed to parse button")
                .into_iter()
                .map(|i| i as usize)
                .collect()
        })
        .collect()
}

fn parse_joltage(s: &str) -> Vec<u64> {
    util::parse_ints_u64(s).expect("failed to parse joltage")
}

fn press_button_p1(state: &mut [bool], button: &[usize]) {
    for &i in button {
        state[i] = !state[i];
    }
}

fn shortest_solve_p1(problem: &Problem) -> u64 {
    let start = vec![false; problem.goal.len()];

    let mut explored = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some((v, n)) = queue.pop_front() {
        if explored.contains(&v) {
            continue;
        } else {
            explored.insert(v.clone());
        }

        if v == problem.goal {
            return n;
        }

        for button in &problem.buttons {
            let mut w = v.clone();
            press_button_p1(&mut w, button);
            queue.push_back((w, n + 1));
        }
    }

    unreachable!()
}

fn shortest_solve_p2(problem: &Problem) -> u64 {
    let mut vars = ProblemVariables::new();

    let mut button_vars = vec![];
    for _button in &problem.buttons {
        button_vars.push(vars.add(variable().min(0).integer()));
    }

    let mut model = vars
        .minimise(button_vars.iter().sum::<good_lp::Expression>())
        .using(default_solver);

    let mut exprs = vec![0.into_expression(); problem.joltage.len()];
    for (i, button) in problem.buttons.iter().enumerate() {
        for &x in button {
            exprs[x] += button_vars[i];
        }
    }
    for (e, &j) in exprs.into_iter().zip(problem.joltage.iter()) {
        model.add_constraint(e.eq(j as f64));
    }

    let solution = model.solve().unwrap();
    let mut sum = 0.0;
    for var in button_vars {
        sum += solution.value(var);
    }

    println!("{sum}");
    sum.round() as u64
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let re = Regex::new(r"\[([.#]+)\]\s([^\{]+)\s\{((?:\d,?)+)\}").unwrap();

    let mut problems: Vec<Problem> = Vec::new();
    for (_, [goal, buttons, joltage]) in re.captures_iter(&input).map(|c| c.extract()) {
        let p = Problem {
            goal: parse_goal(goal),
            buttons: parse_buttons(buttons),
            joltage: parse_joltage(joltage),
        };

        problems.push(p);
    }

    println!("{}", problems.iter().map(shortest_solve_p1).sum::<u64>())
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let re = Regex::new(r"\[([.#]+)\]\s([^\{]+)\s\{((?:\d,?)+)\}").unwrap();

    let mut problems: Vec<Problem> = Vec::new();
    for (_, [goal, buttons, joltage]) in re.captures_iter(&input).map(|c| c.extract()) {
        let p = Problem {
            goal: parse_goal(goal),
            buttons: parse_buttons(buttons),
            joltage: parse_joltage(joltage),
        };

        problems.push(p);
    }

    println!("{}", problems.iter().map(shortest_solve_p2).sum::<u64>())
}
