use std::{collections::HashMap, ops::{IndexMut, Index}};


fn main() {
    do_file("./input.txt");
}

type Num = i64;

fn do_file(filename: &str) -> Num {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

#[derive(Clone, Copy)]
struct Range {
    start: Num,
    end: Num,
}

impl Range {
    fn try_new(start: Num, end: Num) -> Option<Range> {
        (start < end).then_some(Self { start, end })
    }

    fn split(self, n: Num) -> (Option<Self>, Option<Self>) {
        let right_start = n.max(self.start);
        let left_end = n.min(self.end);
        (Self::try_new(self.start, left_end),
            Self::try_new(right_start, self.end))
    }

    fn size(self) -> Num {
        self.end - self.start
    }
}

impl Default for Range {
    fn default() -> Self {
        Self {
            start: 1,
            end: 4001,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Component {
    #[default]
    X,
    M,
    A,
    S
}

impl Component {
    fn value(self) -> usize {
        match self {
            Self::X => 0,
            Self::M => 1,
            Self::A => 2,
            Self::S => 3,
        }
    }
}

impl From<&str> for Component {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Bad component"),
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Part {
    components: [Range; 4],
}

impl Part {
    fn sum(self) -> Num {
        self.components
            .into_iter()
            .map(|c| c.size())
            .product()
    }
}

impl Index<Component> for Part {
    type Output = Range;

    fn index(&self, index: Component) -> &Self::Output {
        &self.components[index.value()]
    }
}

impl IndexMut<Component> for Part {
    fn index_mut(&mut self, index: Component) -> &mut Self::Output {
        &mut self.components[index.value()]
    }
}

// impl From<&str> for Part {
//     fn from(value: &str) -> Self {
//         let end = value.len() - 1;
//         value[1..end]
//             .split(',')
//             .map(|rule| rule.split_once('=').expect("Bad part format"))
//             .map(|(comp, val)| (comp.into(), Num::from_str_radix(val, 10).expect("Can't parse part component value")))
//             .fold(Part::default(), |mut p, (comp, val): (Component, _)| {
//                 p[comp] = val;
//                 p
//             })
//     }
// }

type WorkflowName<'a> = &'a str;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Target<'a> {
    Workflow(WorkflowName<'a>),
    Reject,
    Accept,
}

impl<'a> From<&'a str> for Target<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "R" => Self::Reject,
            "A" => Self::Accept,
            e => Self::Workflow(e),
        }
    }
}

#[derive(Default, Clone, Copy)]
enum Operator {
    #[default]
    NoOp,
    GreaterThan(Num),
    LessThan(Num),
}

impl Operator {
    // fn apply(self, lhs: Num) -> bool {
    //     match self {
    //         Self::NoOp => true,
    //         Self::GreaterThan(rhs) => lhs > rhs,
    //         Self::LessThan(rhs) => lhs < rhs,
    //     }
    // }

    // try to coerce a range so it conforms with the predicate
    // if it doesn't conform - returns None
    fn coerce(self, range: Range) -> (Option<Range>, Option<Range>) {
        match self {
            Self::NoOp => (Some(range), None),
            Self::LessThan(n) => range.split(n),
            Self::GreaterThan(n) => {
                let (rhs, lhs) = range.split(n);
                (lhs, rhs)
            },
        }
    }
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        let (op, val) = value.split_at(1);
        let val = Num::from_str_radix(val, 10).expect("Can't parse op val");
        match op {
            "<" => Self::LessThan(val),
            ">" => Self::GreaterThan(val),
            _ => panic!("Bad operator formatting"),
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Predicate {
    comp: Component,
    op: Operator,
}

impl Predicate {
    fn coerce(self, part: Option<Part>) -> (Option<Part>, Option<Part>) {
        let part = {
            if let Some(part) = part { part }
            else { return (None, None) }
        };

        let range = part[self.comp];
        let (lhs, rhs) = self.op.coerce(range);

        let lhs = lhs.map(|r| {
            let mut part = part.clone();
            part[self.comp] = r;
            part
        });


        let rhs = rhs.map(|r| {
            let mut part = part.clone();
            part[self.comp] = r;
            part
        });

        (lhs, rhs)
    }
}

impl From<&str> for Predicate {
    fn from(value: &str) -> Self {
        if value.contains('<') || value.contains('>') {
            let comp = &value[0..1];
            let opval = &value[1..];

            Self {
                comp: comp.into(),
                op: opval.into(),
            }
        } else {
            Default::default()
        }
    }
}

#[derive(Clone, Copy)]
struct Rule<'a> {
    target: Target<'a>,
    predicate: Predicate,
}

// impl<'a> Rule<'a> {
//     fn apply(self, part: Part) -> Option<Target<'a>> {
//         self.predicate.apply(part).then_some(self.target)
//     }
// }

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        let (predicate, target) = 
            if let Some((p, t)) = value.split_once(':') {
                (p.into(), t.into())
            } else {
                let end = value.len()-1;
                (Default::default(), value[..end].into())
            };

        Self { predicate, target }
    }
}

struct Workflow<'a> {
    name: WorkflowName<'a>,
    rules: Vec<Rule<'a>>,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let (name, rules) = value.split_once('{').expect("Bad workflow format");
        let rules = rules
            .split(',')
            .map(Rule::from)
            .collect();

        Self { name, rules }
    }
}

impl<'a> FromIterator<Workflow<'a>> for HashMap<WorkflowName<'a>, Vec<Rule<'a>>> {
    fn from_iter<T: IntoIterator<Item = Workflow<'a>>>(iter: T) -> Self {
        iter
            .into_iter()
            .map(|w| (w.name, w.rules))
            .collect()
    }
}

// trait WorkflowLike<'a> {
//     fn apply(&self, part: Part) -> Target<'a>;
// }

// impl<'a> WorkflowLike<'a> for HashMap<&'a str, Vec<Rule<'a>>> {
//     fn apply(&self, part: Part) -> Target<'a> {
//         let mut next = "in";
//         'outer: loop {
//             for rule in &self[next] {
//                 if let Some(e) = rule.apply(part) {
//                     match e {
//                         Target::Workspace(w) => {
//                             next = w;
//                             continue 'outer;
//                         },
//                         e => return e,
//                     }
//                 }
//             }
//             panic!("Failed to apply rule")
//         }
//     }
// }

fn find_accepted_ranges<'a>(workflows: &HashMap<&'a str, Vec<Rule<'a>>>) -> Vec<Part> {
    let mut ret = Vec::new();

    let mut stack = Vec::new();
    stack.push((Target::Workflow("in"), Some(Part::default())));
    
    while let Some((target, mut part)) = stack.pop() {
        match target {
            Target::Workflow(w) =>
                for rule in &workflows[w] {
                    if part.is_none() { break }
                    let (coe,rce) = rule.predicate.coerce(part);
                    // if let Some(part) =  coercion.0 {
                    //     stack.push((rule.target, part))
                    // }
                    stack.push((rule.target, coe));
                    part = rce;
                },
            Target::Accept => ret.push(part.expect("Logic error, illegal range yielded accept")),
            Target::Reject => continue,
        }
    }

    ret
}

fn process(input: &str) -> Num {
    let mut it = input.split("\r\n\r\n");
    let workflows = it.next().expect("No workflows")
        .lines()
        .map(Workflow::from)
        .collect::<HashMap<_, _>>()
        ;

    find_accepted_ranges(&workflows)
        .into_iter()
        .map(|range| range.sum())
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(crate::do_file("./example.txt"), 167409079868000);
    }

    #[test]
    fn example2() {
        assert_eq!(crate::do_file("./example2.txt"), 1);
    }
}
