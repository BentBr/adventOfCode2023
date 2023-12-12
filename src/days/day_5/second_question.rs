use crate::days::read_input_into_vector;
use std::cell::RefCell;
use std::iter::from_fn;

pub fn solution() {
    match read_input_into_vector("./src/days/day_5/input") {
        Ok(_lines) => {
            println!(
                "Day 5: 'Seed numbers in pairs (not a pretty, but working solution)' - {}",
                seed_number_calculator()
            );
        }
        Err(err) => {
            println!("\u{001b}[0mCould not load the file: {}", err)
        }
    };
}

fn seed_number_calculator() -> u64 {
    let input = include_bytes!("input");

    let mut seeds = input[7..input.iter().position(|b| b == &b'\n').unwrap()]
        .split(|b| b == &b' ')
        .flat_map(atoi::atoi::<u64>);
    let mut lines = input.split(|b| b == &b'\n').skip(2);

    let maps: Vec<Vec<(std::ops::Range<u64>, u64)>> = (0..7)
        .map(|_| {
            let mut map = (&mut lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut entry = line
                        .splitn(3, |b| b == &b' ')
                        .map(|n| atoi::atoi(n).unwrap());
                    let el: [_; 3] = std::array::from_fn(|_| entry.next().unwrap());
                    (el[1]..el[1] + el[2], el[0])
                })
                .collect::<Vec<_>>();
            map.sort_by_key(|(range, _)| range.start);
            map
        })
        .collect();

    from_fn(|| seeds.next().zip(seeds.next()))
        .map(|(start, len)| start..start + len)
        .flat_map(|range| {
            maps.iter().fold(vec![range], |ranges, map| {
                ranges
                    .into_iter()
                    .flat_map(|base| {
                        let base_cell = RefCell::new(base);
                        map.iter()
                            .take_while(|_| !base_cell.borrow().is_empty())
                            .fold(Vec::with_capacity(6), |mut from, (to, n)| {
                                let mut base = base_cell.borrow_mut();
                                if base.start < to.start {
                                    from.push(base.start..(base.end.min(to.start)));
                                    base.start = to.start;
                                }

                                let len = base.end.min(to.end).saturating_sub(base.start);
                                if len > 0 {
                                    let to = *n + base.start - to.start;
                                    from.push(to..to + len);
                                    base.start += len;
                                }
                                from
                            })
                    })
                    .collect()
            })
        })
        .map(|range| range.start)
        .min()
        .unwrap()
}
