use std::collections::HashMap;

use crate::Day;
// x,m,a,s

type Part = Vec<u64>;
type WorkFlowID<'a> = &'a str;

#[derive(Debug)]
enum Op {
    Greater(u64),
    Less(u64),
}

impl Op {
    fn does_pass(&self, value: u64) -> bool {
        match self {
            Self::Greater(cmp) => value > *cmp,
            Self::Less(cmp) => value < *cmp,
        }
    }
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
                                .map(|(_, cmp)| Op::Greater(cmp.parse().unwrap()))
                                .unwrap()
                        })
                        .unwrap_or_else(|| {
                            condition
                                .split_once('<')
                                .map(|(_, cmp)| Op::Less(cmp.parse().unwrap()))
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
        dbg!(input
            .lines()
            .take_while(|line| line.len() < 3)
            .fold(HashMap::new(), new_workflow));

        todo!()
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".to_string()
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
