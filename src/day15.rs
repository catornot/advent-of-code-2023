use crate::Day;

#[derive(Debug)]
enum Op {
    Equal(u64),
    Dash,
}

#[derive(Debug)]
struct LensBox<'a> {
    label: &'a str,
    focus: u64,
}

pub struct Day15;

impl Day for Day15 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("1320", "145")
    }

    fn part_1(&mut self, input: String) -> String {
        input
            .split(',')
            .map(|line| line.trim().chars().fold(0, hash_char))
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let mut storage = (0..256usize)
            .into_iter()
            .map(|_| Vec::new())
            .collect::<Vec<Vec<LensBox>>>();

        input
            .split(',')
            .map(|line| line.trim())
            .map(|line| {
                line.split_once('=')
                    .map(|(label, focus)| {
                        (
                            label,
                            Op::Equal(focus.parse().expect("should always be correct")),
                        )
                    })
                    .unwrap_or_else(|| {
                        line.split_once('-')
                            .map(|(label, _)| (label, Op::Dash))
                            .expect("should have a dash or equal")
                    })
            })
            .map(|(label, op)| (label, op, label.chars().fold(0, hash_char) as usize))
            .for_each(|(label, op, boxid)| match op {
                Op::Equal(focus) => {
                    let Some(index) = storage[boxid]
                        .iter()
                        .position(|lensbox| lensbox.label == label)
                    else {
                        storage[boxid].push(LensBox { label, focus });
                        return;
                    };

                    storage[boxid][index] = LensBox { label, focus };
                }
                Op::Dash => {
                    let Some(index) = storage[boxid]
                        .iter()
                        .position(|lensbox| lensbox.label == label)
                    else {
                        return;
                    };

                    storage[boxid].remove(index);
                }
            });

        storage
            .into_iter()
            .enumerate()
            .map(|(boxid, row)| (boxid as u64 + 1, row))
            .map(|(boxid, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(i, lens)| (i as u64 + 1, lens))
                    .map(|(i, lensbox)| boxid * i * lensbox.focus)
                    .sum::<u64>()
            })
            .sum::<u64>()
            .to_string()
    }
}

fn hash_char(current_value: u64, c: char) -> u64 {
    (current_value + c as u64) * 17 % 256
}
