use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Item(usize);

#[derive(Clone)]
struct Monkey {
    starting_items: VecDeque<Item>,
    operation: fn(&mut Item),
    test: fn(&Item) -> usize,
    inspected_items: usize,
    divide_by: usize,
}

fn relax(item: &mut Item) {
    item.0 /= 3
}

fn relax_for_part_b(item: &mut Item, value: usize) {
    item.0 %= value
}

fn throw(monkeys: &mut [Monkey], item: Item, destination: usize) {
    monkeys[destination].starting_items.push_back(item);
}

fn calculate_least_common_denominator(monkeys: &[Monkey]) -> usize {
    monkeys.iter().map(|m| m.divide_by).product::<usize>()
}

fn main() {
    let mut monkeys = vec![
        Monkey {
            starting_items: VecDeque::from(vec![Item(98), Item(89), Item(52)]),
            inspected_items: 0,
            operation: |item: &mut Item| item.0 *= 2,
            divide_by: 5,
            test: |item: &Item| {
                if item.0 % 5 == 0 {
                    6
                } else {
                    1
                }
            },
        },
        Monkey {
            starting_items: VecDeque::from(vec![
                Item(57),
                Item(95),
                Item(80),
                Item(92),
                Item(57),
                Item(78),
            ]),
            inspected_items: 0,
            operation: |item: &mut Item| item.0 *= 13,
            divide_by: 2,
            test: |item: &Item| {
                if item.0 % 2 == 0 {
                    2
                } else {
                    6
                }
            },
        },
        Monkey {
            starting_items: VecDeque::from(vec![
                Item(82),
                Item(74),
                Item(97),
                Item(75),
                Item(51),
                Item(92),
                Item(83),
            ]),
            inspected_items: 0,
            operation: |item: &mut Item| item.0 += 5,
            divide_by: 19,
            test: |item: &Item| {
                if item.0 % 19 == 0 {
                    7
                } else {
                    5
                }
            },
        },
        Monkey {
            starting_items: VecDeque::from(vec![Item(97), Item(88), Item(51), Item(68), Item(76)]),
            inspected_items: 0,
            operation: |item: &mut Item| item.0 += 6,
            divide_by: 7,
            test: |item: &Item| {
                if item.0 % 7 == 0 {
                    0
                } else {
                    4
                }
            },
        },
        Monkey {
            starting_items: VecDeque::from(vec![Item(63)]),
            inspected_items: 0,
            operation: |item: &mut Item| item.0 += 1,
            divide_by: 17,
            test: |item: &Item| usize::from(item.0 % 17 != 0),
        },
        Monkey {
            starting_items: VecDeque::from(vec![Item(94), Item(91), Item(51), Item(63)]),
            inspected_items: 0,
            operation: |item: &mut Item| item.0 += 4,
            divide_by: 13,
            test: |item: &Item| {
                if item.0 % 13 == 0 {
                    4
                } else {
                    3
                }
            },
        },
        Monkey {
            starting_items: VecDeque::from(vec![
                Item(61),
                Item(54),
                Item(94),
                Item(71),
                Item(74),
                Item(68),
                Item(98),
                Item(83),
            ]),
            inspected_items: 0,
            operation: |item: &mut Item| item.0 += 2,
            divide_by: 3,
            test: |item: &Item| {
                if item.0 % 3 == 0 {
                    2
                } else {
                    7
                }
            },
        },
        Monkey {
            starting_items: VecDeque::from(vec![Item(90), Item(56)]),
            inspected_items: 0,
            operation: |item: &mut Item| item.0 *= item.0,
            divide_by: 11,
            test: |item: &Item| {
                if item.0 % 11 == 0 {
                    3
                } else {
                    5
                }
            },
        },
    ];

    let mut monkeys_2 = monkeys.clone();

    for _ in 0..20 {
        for i in 0..8 {
            while !monkeys[i].starting_items.is_empty() {
                let mut current_item = monkeys[i].starting_items.pop_front().unwrap();

                (monkeys[i].operation)(&mut current_item);

                monkeys[i].inspected_items += 1;

                relax(&mut current_item);

                let destination = (monkeys[i].test)(&current_item);

                throw(&mut monkeys, current_item, destination);
            }
        }
    }

    let mut business: Vec<usize> = monkeys.iter().map(|m| m.inspected_items).collect();
    business.sort();
    println!(
        "Business Value: {}",
        business.iter().rev().take(2).product::<usize>()
    );

    let least_common_denominator = calculate_least_common_denominator(&monkeys);

    for _ in 0..10_000 {
        for i in 0..8 {
            while !monkeys_2[i].starting_items.is_empty() {
                let mut current_item = monkeys_2[i].starting_items.pop_front().unwrap();

                (monkeys_2[i].operation)(&mut current_item);

                monkeys_2[i].inspected_items += 1;

                // relax(&mut current_item);
                relax_for_part_b(&mut current_item, least_common_denominator);

                let destination = (monkeys_2[i].test)(&current_item);

                throw(&mut monkeys_2, current_item, destination);
            }
        }
    }

    let mut business: Vec<usize> = monkeys_2.iter().map(|m| m.inspected_items).collect();
    business.sort();
    println!(
        "Business Value: {}",
        business.iter().rev().take(2).product::<usize>()
    );
}
