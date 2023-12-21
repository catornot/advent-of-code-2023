use core::panic;
use std::collections::HashMap;

use crate::Day;
// x,m,a,s

type Part = Vec<u64>;
type WorkFlowID<'a> = &'a str;

#[derive(Debug)]
enum Op {
    Greater(usize, u64),
    Less(usize, u64),
}

impl Op {
    fn does_pass(&self, part: &Part) -> bool {
        match self {
            Self::Greater(index, cmp) => part[*index] > *cmp,
            Self::Less(index, cmp) => part[*index] < *cmp,
        }
    }
}

enum TaskProcessed<'a> {
    Reject,
    Accept,
    Send(WorkFlowID<'a>),
    None,
}

#[derive(Debug)]
enum Task<'a> {
    Condition(Op, Box<Task<'a>>),
    Reject,
    Accept,
    Send(WorkFlowID<'a>),
}

impl<'a> Task<'a> {
    fn new(task: &'a str) -> Self {
        match task {
            task if task.contains('<') || task.contains('>') => {
                let (condition, jump) = task
                    .split_once(':')
                    .map(|(condition, jump)| (condition, Box::new(Task::new(jump))))
                    .unwrap();
                Self::Condition(
                    condition
                        .contains('>')
                        .then(|| {
                            condition
                                .split_once('>')
                                .map(|(symbol, cmp)| {
                                    Op::Greater(symbol_to_index(symbol), cmp.parse().unwrap())
                                })
                                .unwrap()
                        })
                        .unwrap_or_else(|| {
                            condition
                                .split_once('<')
                                .map(|(symbol, cmp)| {
                                    Op::Less(symbol_to_index(symbol), cmp.parse().unwrap())
                                })
                                .unwrap()
                        }),
                    jump,
                )
            }
            task if task == "A" => Self::Accept,
            task if task == "R" => Self::Reject,
            jump => Self::Send(jump),
        }
    }
}

pub struct Day19;

impl Day for Day19 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#,
            "",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("19114", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let workflow = input
            .lines()
            .take_while(|line| line.len() >= 3)
            .fold(HashMap::new(), new_workflow);

        let mut reached_parts = false;
        input
            .lines()
            .skip_while(|line| {
                !((line.len() < 3).then(|| reached_parts = true).is_some() || reached_parts)
            })
            .filter(|line| line.len() > 2)
            .map(|part| {
                part[1..part.len() - 1]
                    .split(',')
                    .map(|value| {
                        value
                            .split_once('=')
                            .expect("parts should have a = sign")
                            .1
                            .parse()
                            .expect("parts should have a number")
                    })
                    .collect::<Part>()
            })
            .filter(|part| is_accpeted(part, &workflow))
            .map(|part| part.into_iter().sum::<u64>())
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".to_string()
    }
}

fn is_accpeted<'a>(part: &Part, workflows: &HashMap<&'a str, Vec<Task<'a>>>) -> bool {
    let mut next_id = "in";
    loop {
        let tasks = workflows
            .get(next_id)
            .expect("workflow should contain every id");

        for task in tasks.into_iter() {
            match process_task(task, part) {
                TaskProcessed::Reject => return false,
                TaskProcessed::Accept => return true,
                TaskProcessed::Send(id) => {
                    next_id = id;
                    break;
                }
                TaskProcessed::None => {}
            }
        }
    }
}

fn process_task<'a>(task: &'a Task<'a>, part: &Part) -> TaskProcessed<'a> {
    match task {
        Task::Reject => TaskProcessed::Reject,
        Task::Accept => TaskProcessed::Accept,
        Task::Condition(op, next_task) => op
            .does_pass(part)
            .then(|| process_task(&next_task, part))
            .unwrap_or_else(|| TaskProcessed::None),
        Task::Send(id) => TaskProcessed::Send(id),
    }
}

fn new_workflow<'a>(
    mut workflows: HashMap<&'a str, Vec<Task<'a>>>,
    line: &'a str,
) -> HashMap<&'a str, Vec<Task<'a>>> {
    let (id, tasks) = line
        .split_once('{')
        .expect("workflows should have a id and tasks");

    workflows.insert(
        id,
        tasks
            .split_once('}')
            .map(|t| t.0)
            .unwrap_or_else(|| tasks)
            .split(',')
            .map(Task::new)
            .collect(),
    );
    workflows
}
fn symbol_to_index(symbol: &str) -> usize {
    match symbol {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("invalid symbol"),
    }
}
