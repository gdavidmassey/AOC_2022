use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Monkey {
    items: VecDeque<usize>,
    test: usize,
    operation: fn(usize) -> usize,
    true_monkey: usize,
    false_monkey: usize,
    inspected: usize,
}

pub enum PuzzlePart {
    One,
    Two,
}

impl Monkey {
    fn throw(&mut self, catcher: &[Rc<RefCell<Monkey>>]) {
        while !self.items.is_empty() {
            self.inspected += 1;
            let mut worry = self.items.pop_front().unwrap();
            worry = (self.operation)(worry);
            worry /= 3;
            match worry % self.test == 0 {
                true => {
                    let mut catcher = catcher[self.true_monkey].borrow_mut();
                    catcher.items.push_back(worry);
                }
                false => {
                    let mut catcher = catcher[self.false_monkey].borrow_mut();
                    catcher.items.push_back(worry);
                }
            }
        }
    }

    fn throw_2(&mut self, catcher: &[Rc<RefCell<Monkey>>]) {
        while !self.items.is_empty() {
            self.inspected += 1;
            let mut worry = self.items.pop_front().unwrap();
            worry = (self.operation)(worry);
            worry %= 9699690;
            match worry % self.test == 0 {
                true => {
                    let mut catcher = catcher[self.true_monkey].borrow_mut();
                    catcher.items.push_back(worry);
                }
                false => {
                    let mut catcher = catcher[self.false_monkey].borrow_mut();
                    catcher.items.push_back(worry);
                }
            }
        }
    }
}
pub fn day_11(part: PuzzlePart) -> usize {
    let monkey0 = Monkey {
        items: VecDeque::from(vec![78, 53, 89, 51, 52, 59, 58, 85]),
        test: 5,
        operation: |x| -> usize { x * 3 },
        true_monkey: 2,
        false_monkey: 7,
        inspected: 0,
    };
    let monkey1 = Monkey {
        items: VecDeque::from(vec![64]),
        test: 2,
        operation: |x| -> usize { x + 7 },
        true_monkey: 3,
        false_monkey: 6,
        inspected: 0,
    };
    let monkey2 = Monkey {
        items: VecDeque::from(vec![71, 93, 65, 82]),
        test: 13,
        operation: |x| -> usize { x + 5 },
        true_monkey: 5,
        false_monkey: 4,
        inspected: 0,
    };
    let monkey3 = Monkey {
        items: VecDeque::from(vec![67, 73, 95, 75, 56, 74]),
        test: 19,
        operation: |x| -> usize { x + 8 },
        true_monkey: 6,
        false_monkey: 0,
        inspected: 0,
    };
    let monkey4 = Monkey {
        items: VecDeque::from(vec![85, 91, 90]),
        test: 11,
        operation: |x| -> usize { x + 4 },
        true_monkey: 3,
        false_monkey: 1,
        inspected: 0,
    };
    let monkey5 = Monkey {
        items: VecDeque::from(vec![67, 96, 69, 55, 70, 83, 62]),
        test: 3,
        operation: |x| -> usize { x * 2 },
        true_monkey: 4,
        false_monkey: 1,
        inspected: 0,
    };
    let monkey6 = Monkey {
        items: VecDeque::from(vec![53, 86, 98, 70, 64]),
        test: 7,
        operation: |x| -> usize { x + 6 },
        true_monkey: 7,
        false_monkey: 0,
        inspected: 0,
    };
    let monkey7 = Monkey {
        items: VecDeque::from(vec![88, 64]),
        test: 17,
        operation: |x| -> usize { x * x },
        true_monkey: 2,
        false_monkey: 5,
        inspected: 0,
    };

    let monkeys = vec![
        Rc::new(RefCell::new(monkey0)),
        Rc::new(RefCell::new(monkey1)),
        Rc::new(RefCell::new(monkey2)),
        Rc::new(RefCell::new(monkey3)),
        Rc::new(RefCell::new(monkey4)),
        Rc::new(RefCell::new(monkey5)),
        Rc::new(RefCell::new(monkey6)),
        Rc::new(RefCell::new(monkey7)),
    ];

    match part {
        PuzzlePart::One => {
            for _ in 0..20 {
                for i in 0..8usize {
                    monkeys[i].borrow_mut().throw(&monkeys[..]);
                }
            }
        }
        PuzzlePart::Two => {
            for _ in 0..10000 {
                for i in 0..8usize {
                    monkeys[i].borrow_mut().throw_2(&monkeys[..]);
                }
            }
        }
    };

    let mut inspections = Vec::new();

    for monkey in &monkeys {
        inspections.push(monkey.borrow().inspected);
    }
    inspections.sort_by(|a, b| b.cmp(a));
    let answer = inspections[0] * inspections[1];
    println!("{}", answer);
    answer
}
