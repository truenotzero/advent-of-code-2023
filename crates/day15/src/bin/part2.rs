use std::num::Wrapping;


fn main() {
    // first try!
    do_file("./input.txt");
}

type Num = i32;

fn do_file(filename: &str) -> Num {
    let input = std::fs::read_to_string(filename).unwrap();
    let output = process(&input);
    println!("{output}");
    output
}

fn hash(input: &str) -> Wrapping<u8> {
    input
        .as_bytes()
        .into_iter()
        .fold(Wrapping(0), |mut acc, &b| {
            acc += b;
            acc *= 17;
            acc
        })
}

struct Lens<'a> {
    label: &'a str,
    focal_len: Num,
}

enum Op {
    Remove,
    Set(Num),
}

impl<'a> TryFrom<&'a str> for Op {
    type Error=&'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match &value[0..1] {
            "-" => Ok(Op::Remove),
            "=" => {
                let num = Num::from_str_radix(&value[1..], 10).or(Err(value))?;
                Ok(Op::Set(num))
            },
            _ => Err(value),
        }
    }
}

struct Action<'a> {
    label: &'a str,
    op: Op,
}

impl<'a> TryFrom<&'a str> for Action<'a> {
    type Error=&'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let label = value.split(['-', '=']).next().expect("Bad label formatting");
        let oparg = &value[label.len()..];
        Ok(Self {
            label,
            op: oparg.try_into()?,
        })
    }
}

fn box_focusing_power<'a, I: IntoIterator<Item=Lens<'a>>>(box_idx: usize, it: I) -> Num {
    it
        .into_iter()
        .enumerate()
        .map(|(slot_idx, lens)| (box_idx + 1) * (slot_idx + 1) * (lens.focal_len as usize))
        .sum::<usize>()
        as Num
}

fn process<'a>(input: &'a str) -> Num {
    let hashmap = [(); 256].map(|_| Vec::<Lens<'a>>::new());

    input
        .split(",")
        .map(|e| e.try_into().expect("Cannot parse action!"))
        .fold(hashmap, |mut map, action: Action| {
            let idx = hash(action.label).0 as usize;
            match action.op {
                Op::Remove => map[idx].retain(|e| e.label != action.label),
                Op::Set(focal_len) => {
                    let mut replaced = false;
                    for e in &mut map[idx] {
                        if e.label == action.label {
                             e.focal_len = focal_len;
                             replaced = true;
                        }
                    }

                    if !replaced {
                        map[idx].push(Lens { label: action.label, focal_len });
                    }
                },
            }

            map
        })
        .into_iter()
        .enumerate()
        .map(|(box_idx, b)| box_focusing_power(box_idx, b))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        // first try!
        assert_eq!(crate::do_file("./example.txt"), 145);
    }
}
