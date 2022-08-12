use std::{cell::RefCell, collections::HashSet, rc::Rc};

enum StateType {
    Start,
    Normal,
    End,
}

enum Condition {
    Epsilon,
    Char(char),
    Wildcard,
}

struct State {
    id: usize,
    state_type: StateType,
    transitions: Vec<(Condition, Rc<RefCell<State>>)>,
}

impl State {
    fn new(id: &mut usize, state_type: StateType) -> Rc<RefCell<Self>> {
        let state = Rc::new(RefCell::new(Self {
            id: *id,
            state_type,
            transitions: vec![],
        }));
        *id += 1;
        state
    }

    fn add(&mut self, condition: Condition, next: Rc<RefCell<State>>) {
        self.transitions.push((condition, next));
    }

    fn build(pattern: &Vec<char>) -> Rc<RefCell<Self>> {
        let mut id = 0;
        let start = Self::new(&mut id, StateType::Start);

        let mut tail = start.clone();
        let mut p = 0;
        while p < pattern.len() {
            let c = pattern[p];
            let condition = if c == '.' {
                Condition::Wildcard
            } else {
                Condition::Char(c)
            };
            let multiple = p + 1 < pattern.len() && pattern[p + 1] == '*';

            let l = Self::new(&mut id, StateType::Normal);
            let r = Self::new(&mut id, StateType::Normal);
            l.borrow_mut().add(condition, r.clone());

            if multiple {
                l.borrow_mut().add(Condition::Epsilon, r.clone());
                r.borrow_mut().add(Condition::Epsilon, l.clone());
            }

            tail.borrow_mut().add(Condition::Epsilon, l);
            tail = r.clone();

            p += if multiple { 2 } else { 1 };
        }
        tail.borrow_mut()
            .add(Condition::Epsilon, Self::new(&mut id, StateType::End));
        drop(tail);

        start
    }
}

struct Solution;

impl Solution {
    fn to_match(
        state: Rc<RefCell<State>>,
        pointer: usize,
        chars: &Vec<char>,
        rem: &mut HashSet<(usize, usize)>,
    ) -> bool {
        let state = state.borrow();
        let situation = (state.id, pointer);
        if rem.contains(&situation) {
            false
        } else {
            rem.insert(situation);
            match state.state_type {
                StateType::Start | StateType::Normal => {
                    state
                        .transitions
                        .iter()
                        .any(|(condition, next_state)| match condition {
                            Condition::Epsilon => {
                                Self::to_match(next_state.clone(), pointer, chars, rem)
                            }
                            Condition::Char(c) => {
                                pointer < chars.len()
                                    && chars[pointer] == *c
                                    && Self::to_match(next_state.clone(), pointer + 1, chars, rem)
                            }
                            Condition::Wildcard => {
                                pointer < chars.len()
                                    && Self::to_match(next_state.clone(), pointer + 1, chars, rem)
                            }
                        })
                }
                StateType::End => pointer == chars.len(),
            }
        }
    }

    pub fn is_match(s: String, p: String) -> bool {
        Self::to_match(
            State::build(&p.chars().collect()),
            0,
            &s.chars().collect(),
            &mut HashSet::new(),
        )
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_match("abccd".to_string(), "a.c*d".to_string())
    )
}
