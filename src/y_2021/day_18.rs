use std::ptr::eq;
use std::{fmt::Display, str::Chars};

use itertools::{Itertools, PutBackN};
use rand::Rng;

pub fn entry() {
    println!("Starting day 18!");

    let snailfish_numbers = aoc::read_input("./resources/y_2021/day_18_input.txt", move |line| {
        return SnailfishNumberNode::new(line);
    });

    /*for snailfish_number in snailfish_numbers.iter() {
        println!("{}", snailfish_number);
    }*/

    /*let test1 = SnailfishNumberNode::new("[1,2]");
    let test2 = SnailfishNumberNode::new("[[3,4],5]");
    println!("{}", test1.add(&test2));*/

    /*let test_explode_1 = SnailfishNumberNode::new("[[3,4],5]");
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![]).1);
    let test_explode_1 = SnailfishNumberNode::new("[[[[[9,8],1],2],3],4]");
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![]).1);
    let test_explode_1 = SnailfishNumberNode::new("[7,[6,[5,[4,[3,2]]]]]");
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![]).1);
    let test_explode_1 = SnailfishNumberNode::new("[[6,[5,[4,[3,2]]]],1]");
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![]).1);
    let test_explode_1 = SnailfishNumberNode::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![]).1);
    let test_explode_1 = SnailfishNumberNode::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![]).1);
    let test_explode_1 = SnailfishNumberNode::new("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"); // [[[[0,7],4],[15,[0,13]]],[1,1]]
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![]).1);
    let test_explode_1 = SnailfishNumberNode::new(
        "[[[[4,0],[0,0]],[[[4,5],[2,6]],[9,5]]],[12,[[[3,7],[4,3]],[[6,3],[8,8]]]]]", // [[[[4,0],[0,4]],[[0,[7,6]],[9,5]]],[12,[[[3,7],[4,3]],[[6,3],[8,8]]]]]
    );
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![]).1);*/
    /*let test_explode_1 = SnailfishNumberNode::new(
        "[[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]],[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]]", // [[[[4,0],[0,4]],[[0,[7,6]],[9,5]]],[12,[[[3,7],[4,3]],[[6,3],[8,8]]]]]
    );
    println!("{}", test_explode_1.explode(&mut 0, &mut vec![], true).1);*/

    /*let test_split = SnailfishNumberNode::new("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    println!("{}, {}", test_split.split().0, test_split.split().1);
    let test_split = SnailfishNumberNode::new("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    println!("{}, {}", test_split.split().0, test_split.split().1);
    let test_split = SnailfishNumberNode::new("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    println!("{}, {}", test_split.split().0, test_split.split().1);*/

    /*let test_reduce = SnailfishNumberNode::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    println!("{}", test_reduce.reduce());*/

    /*let test1 = SnailfishNumberNode::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let test2 = SnailfishNumberNode::new("[1,1]");
    println!("{}", test1.add(&test2));*/

    /*let test1 =
        SnailfishNumberNode::new("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
    let test2 = SnailfishNumberNode::new("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]");
    println!("{}", test1.add(&test2, false));*/

    /*let test_mag = SnailfishNumberNode::new("[[1,2],[[3,4],5]]");
    println!("{}", test_mag.magnitude());
    let test_mag = SnailfishNumberNode::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    println!("{}", test_mag.magnitude());
    let test_mag = SnailfishNumberNode::new("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    println!("{}", test_mag.magnitude());
    let test_mag = SnailfishNumberNode::new("[[[[3,0],[5,3]],[4,4]],[5,5]]");
    println!("{}", test_mag.magnitude());
    let test_mag = SnailfishNumberNode::new("[[[[5,0],[7,4]],[5,5]],[6,6]]");
    println!("{}", test_mag.magnitude());
    let test_mag =
        SnailfishNumberNode::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    println!("{}", test_mag.magnitude());*/

    let mut add_result = snailfish_numbers[0].clone();
    for (index, snailfish_number) in snailfish_numbers.iter().enumerate() {
        if index == 0 {
            continue;
        }

        add_result = add_result.add(snailfish_number, false);
        println!("Add result: {}", add_result);
    }

    println!("{}", add_result);
    println!("{}", add_result.magnitude());

    let mut largest_mag = 0;
    for i in snailfish_numbers.iter() {
        for j in snailfish_numbers.iter() {
            if i == j {
                continue;
            }

            let mag = i.add(j, false).magnitude();

            if mag > largest_mag {
                largest_mag = mag;
            }

            let mag = j.add(i, false).magnitude();

            if mag > largest_mag {
                largest_mag = mag;
            }
        }
    }

    println!("{}", largest_mag);
}

#[derive(Debug)]
enum SnailfishNumberParserState {
    Start,
    Left,
    Right,
}

#[derive(Debug)]
enum SnailfishNumberExplosionState {
    None,
    Exploding,
    ExplodedLeft,
    ExplodedRight,
    Done,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SnailfishNumberNode {
    left_number: Option<u32>,
    right_number: Option<u32>,
    left_node: Option<Box<SnailfishNumberNode>>,
    right_node: Option<Box<SnailfishNumberNode>>,
    id: u32,
}

impl SnailfishNumberNode {
    fn new<T: AsRef<str>>(str: T) -> SnailfishNumberNode {
        let mut str_iter = itertools::put_back_n(str.as_ref().chars());
        Self::parse(&mut str_iter)
    }

    fn parse(chars: &mut PutBackN<Chars>) -> SnailfishNumberNode {
        let mut state = SnailfishNumberParserState::Start;
        let mut left_num = "".to_string();
        let mut right_num = "".to_string();
        let mut left_node: Option<Box<SnailfishNumberNode>> = None;
        let mut right_node: Option<Box<SnailfishNumberNode>> = None;

        loop {
            let current_char = chars.next();
            //println!("{}", current_char.unwrap());

            match state {
                SnailfishNumberParserState::Start => match current_char {
                    Some('[') => state = SnailfishNumberParserState::Left,
                    Some(invalid) => {
                        eprintln!("Invalid input! At {:?}, but received {}", state, invalid);
                        panic!();
                    }
                    None => {
                        eprintln!(
                            "Invalid input! At {:?}, but reached the end of the iterator",
                            state
                        );
                        panic!();
                    }
                },
                SnailfishNumberParserState::Left => match current_char {
                    Some('0'..='9') => {
                        state = SnailfishNumberParserState::Left;
                        left_num.push(current_char.unwrap())
                    }
                    Some(',') => state = SnailfishNumberParserState::Right,
                    Some('[') => {
                        chars.put_back('[');
                        left_node = Some(Box::new(SnailfishNumberNode::parse(chars)));
                    }
                    Some(invalid) => {
                        eprintln!("Invalid input! At {:?}, but received {}", state, invalid);
                        panic!();
                    }
                    None => {
                        eprintln!(
                            "Invalid input! At {:?}, but reached the end of the iterator",
                            state
                        );
                        panic!();
                    }
                },
                SnailfishNumberParserState::Right => match current_char {
                    Some('0'..='9') => {
                        state = SnailfishNumberParserState::Right;
                        right_num.push(current_char.unwrap())
                    }
                    Some('[') => {
                        chars.put_back('[');
                        right_node = Some(Box::new(SnailfishNumberNode::parse(chars)));
                    }
                    Some(']') => {
                        break;
                    }
                    Some(invalid) => {
                        eprintln!("Invalid input! At {:?}, but received {}", state, invalid);
                        panic!();
                    }
                    None => {
                        eprintln!(
                            "Invalid input! At {:?}, but reached the end of the iterator",
                            state
                        );
                        panic!();
                    }
                },
            }
        }

        SnailfishNumberNode {
            left_number: match left_num.as_str() {
                "" => None,
                _ => Some(left_num.parse().unwrap()),
            },
            right_number: match right_num.as_str() {
                "" => None,
                _ => Some(right_num.parse().unwrap()),
            },
            left_node,
            right_node,
            id: rand::thread_rng().gen_range(0..u32::MAX),
        }
    }

    fn add(&self, other: &Self, debug: bool) -> SnailfishNumberNode {
        let add = SnailfishNumberNode {
            left_number: None,
            right_number: None,
            left_node: Some(Box::new(self.clone())),
            right_node: Some(Box::new(other.clone())),
            id: rand::thread_rng().gen_range(0..u32::MAX),
        };

        //println!("{}", add);

        add.reduce(debug)
    }

    fn stringify(&self) -> String {
        let mut self_str = "[".to_string();

        if self.left_node.is_some() {
            self_str.push_str(&self.left_node.as_ref().unwrap().as_ref().stringify());
        } else {
            self_str.push_str(&self.left_number.unwrap().to_string());
        }

        self_str.push(',');

        if self.right_node.is_some() {
            self_str.push_str(&self.right_node.as_ref().unwrap().as_ref().stringify());
        } else {
            self_str.push_str(&self.right_number.unwrap().to_string());
        }

        self_str.push(']');

        return self_str;
    }

    fn explode(
        &self,
        level: &mut u32,
        last_nodes: &mut Vec<SnailfishNumberNode>,
        debug: bool,
    ) -> (bool, SnailfishNumberNode) {
        if self.left_number.is_some() && *level >= 4 {
            return (
                true,
                SnailfishNumberNode::build_from_explode(self.clone(), true, last_nodes, debug),
            );

            /*let last = last_nodes.last().unwrap();

            if last.left_number.is_some() && last.right_node.is_some() {
                return (
                    true,
                    SnailfishNumberNode::build_from_explode(self.clone(), true, last_nodes),
                );
            }

            if last.left_node.is_some() && last.right_number.is_some() {
                return (
                    true,
                    SnailfishNumberNode::build_from_explode(self.clone(), false, last_nodes),
                );
            }

            // TODO how to know which one here??*/
        }

        if self.left_node.is_some() && *level < 4 {
            *level += 1;
            last_nodes.push(self.clone());

            let explosion = self
                .left_node
                .as_ref()
                .unwrap()
                .as_ref()
                .explode(level, last_nodes, debug);

            if explosion.0 {
                return explosion;
            }

            last_nodes.pop();
        }

        if self.right_number.is_some() && *level >= 4 {
            return (
                true,
                SnailfishNumberNode::build_from_explode(self.clone(), true, last_nodes, debug),
            );

            /*let last = last_nodes.last().unwrap();

            if last.left_number.is_some() && last.right_node.is_some() {
                return (
                    true,
                    SnailfishNumberNode::build_from_explode(self.clone(), true, last_nodes),
                );
            }

            if last.left_node.is_some() && last.right_number.is_some() {
                return (
                    true,
                    SnailfishNumberNode::build_from_explode(self.clone(), false, last_nodes),
                );
            }

            // TODO how to know which one here??*/
        }

        if self.right_node.is_some() && *level < 4 {
            *level += 1;
            last_nodes.push(self.clone());

            let explosion = self
                .right_node
                .as_ref()
                .unwrap()
                .as_ref()
                .explode(level, last_nodes, debug);

            if explosion.0 {
                return explosion;
            }

            last_nodes.pop();
        }

        *level = level.saturating_sub(1);

        (false, self.clone())
    }

    fn build_from_explode(
        node_to_explode: SnailfishNumberNode,
        pos_in_last: bool, // false: left, true: right
        node_stack: &Vec<SnailfishNumberNode>,
        debug: bool,
    ) -> SnailfishNumberNode {
        let mut to_return = node_to_explode.clone();
        let mut explosion_state = SnailfishNumberExplosionState::Exploding;

        if debug {
            println!("Starting explosion of {}", node_to_explode);
        }

        let mut last_closest_node = node_stack.last().unwrap();
        let mut closest_node_iter = node_stack.iter().rev();
        let mut last_copy = false;
        loop {
            let closest_node = match closest_node_iter.next() {
                Some(node) => node,
                None => break,
            };
            if debug {
                println!("Closest node is {}", closest_node);
            }

            if closest_node.left_number.is_some() {
                match explosion_state {
                    SnailfishNumberExplosionState::Exploding => {
                        to_return = SnailfishNumberNode {
                            left_number: Some(
                                closest_node.left_number.unwrap()
                                    + node_to_explode.left_number.unwrap(),
                            ),
                            left_node: None,
                            right_number: Some(0),
                            right_node: None,
                            id: rand::thread_rng().gen_range(0..u32::MAX),
                        };

                        if debug {
                            println!(
                                "State exploding and found left number. New node is {}",
                                to_return
                            );
                        }

                        explosion_state = SnailfishNumberExplosionState::ExplodedLeft;
                        last_closest_node = closest_node;

                        continue;
                    }
                    SnailfishNumberExplosionState::ExplodedRight => {
                        to_return = SnailfishNumberNode {
                            left_number: Some(
                                closest_node.left_number.unwrap()
                                    + node_to_explode.left_number.unwrap(),
                            ),
                            left_node: None,
                            right_number: None,
                            right_node: Some(Box::new(to_return)),
                            id: rand::thread_rng().gen_range(0..u32::MAX),
                        };

                        if debug {
                            println!(
                                "State exploded right and found left number. New node is {}",
                                to_return
                            );
                        }

                        explosion_state = SnailfishNumberExplosionState::Done;
                        last_closest_node = closest_node;

                        continue;

                        //break;
                    }
                    /*SnailfishNumberExplosionState::ExplodedLeft => {
                        to_return = SnailfishNumberNode {
                            left_number: None,
                            left_node: Some(Box::new(to_return)),
                            right_number: None,
                            right_node: Some(Box::new(SnailfishNumberNode {
                                left_number: Some(
                                    closest_node.left_number.unwrap()
                                        + node_to_explode.right_number.unwrap(),
                                ),
                                left_node: None,
                                right_node: closest_node.right_node.clone(),
                                right_number: closest_node.right_number.clone(),
                            })),
                        };

                        println!(
                            "State exploded left and found left number. New node is {}",
                            to_return
                        );

                        last_closest_node = closest_node;

                        continue;
                    }*/
                    _ => (),
                }
            }

            if closest_node.right_number.is_some() {
                match explosion_state {
                    SnailfishNumberExplosionState::Exploding => {
                        to_return = SnailfishNumberNode {
                            left_number: Some(0),
                            left_node: None,
                            right_number: Some(
                                closest_node.right_number.unwrap()
                                    + node_to_explode.right_number.unwrap(),
                            ),
                            right_node: None,
                            id: rand::thread_rng().gen_range(0..u32::MAX),
                        };

                        if debug {
                            println!(
                                "State exploding and found right number. New node is {}",
                                to_return
                            );
                        }

                        explosion_state = SnailfishNumberExplosionState::ExplodedRight;
                        last_closest_node = closest_node;

                        continue;
                    }
                    SnailfishNumberExplosionState::ExplodedLeft => {
                        to_return = SnailfishNumberNode {
                            left_number: None,
                            left_node: Some(Box::new(to_return)),
                            right_number: Some(
                                closest_node.right_number.unwrap()
                                    + node_to_explode.right_number.unwrap(),
                            ),
                            right_node: None,
                            id: rand::thread_rng().gen_range(0..u32::MAX),
                        };

                        if debug {
                            println!(
                                "State exploded left and found right number. New node is {}",
                                to_return
                            );
                        }

                        explosion_state = SnailfishNumberExplosionState::Done;
                        last_closest_node = closest_node;

                        continue;

                        //break;
                    }
                    /*SnailfishNumberExplosionState::ExplodedRight => {
                        to_return = SnailfishNumberNode {
                            left_number: None,
                            left_node: Some(Box::new(SnailfishNumberNode {
                                left_number: closest_node.left_number.clone(),
                                left_node: closest_node.left_node.clone(),
                                right_node: None,
                                right_number: Some(
                                    closest_node.right_number.unwrap()
                                        + node_to_explode.left_number.unwrap(),
                                ),
                            })),
                            right_number: None,
                            right_node: Some(Box::new(to_return)),
                        };

                        println!(
                            "State exploded right and found right number. New node is {}",
                            to_return
                        );

                        last_closest_node = closest_node;

                        continue;
                    }*/
                    _ => (),
                }
            }

            if closest_node.left_node.is_some()
                && closest_node.left_node.as_ref().unwrap().as_ref() != &node_to_explode
            {
                match explosion_state {
                    SnailfishNumberExplosionState::Exploding => {
                        to_return = SnailfishNumberNode {
                            left_node: Some(Box::new(
                                closest_node
                                    .left_node
                                    .as_ref()
                                    .unwrap()
                                    .as_ref()
                                    .add_rightmost(node_to_explode.left_number.unwrap())
                                    .1,
                            )),
                            left_number: None,
                            right_node: None,
                            right_number: Some(0),
                            id: rand::thread_rng().gen_range(0..u32::MAX),
                        };

                        if debug {
                            println!("Found left node. New node is {}", to_return);
                        }

                        explosion_state = SnailfishNumberExplosionState::ExplodedLeft;
                        last_closest_node = closest_node;
                        continue;
                    }
                    /*SnailfishNumberExplosionState::ExplodedRight => {
                        to_return = SnailfishNumberNode {
                            left_node: Some(Box::new(
                                closest_node
                                    .left_node
                                    .as_ref()
                                    .unwrap()
                                    .as_ref()
                                    .add_rightmost(node_to_explode.left_number.unwrap())
                                    .1,
                            )),
                            left_number: None,
                            right_node: Some(Box::new(to_return.clone())),
                            right_number: None,
                        };

                        println!("Found left node. New node is {}", to_return);

                        explosion_state = SnailfishNumberExplosionState::Done;
                        last_closest_node = closest_node;
                        continue;
                    }*/
                    _ => (),
                }
            }

            if closest_node.right_node.is_some()
                && closest_node.right_node.as_ref().unwrap().as_ref() != &node_to_explode
            {
                match explosion_state {
                    SnailfishNumberExplosionState::Exploding => {
                        to_return = SnailfishNumberNode {
                            left_node: None,
                            left_number: Some(0),
                            right_node: Some(Box::new(
                                closest_node
                                    .right_node
                                    .as_ref()
                                    .unwrap()
                                    .as_ref()
                                    .add_leftmost(node_to_explode.right_number.unwrap())
                                    .1,
                            )),
                            right_number: None,
                            id: rand::thread_rng().gen_range(0..u32::MAX),
                        };

                        if debug {
                            println!("Found right node. New node is {}", to_return);
                        }

                        explosion_state = SnailfishNumberExplosionState::ExplodedRight;
                        last_closest_node = closest_node;
                        continue;
                    }
                    /*SnailfishNumberExplosionState::ExplodedLeft => {
                        to_return = SnailfishNumberNode {
                            left_node: Some(Box::new(to_return.clone())),
                            left_number: None,
                            right_node: Some(Box::new(
                                closest_node
                                    .right_node
                                    .as_ref()
                                    .unwrap()
                                    .as_ref()
                                    .add_leftmost(node_to_explode.right_number.unwrap())
                                    .1,
                            )),
                            right_number: None,
                        };

                        explosion_state = SnailfishNumberExplosionState::Done;
                        last_closest_node = closest_node;
                        continue;
                    }*/
                    _ => (),
                }
            }

            if closest_node.left_node.is_some()
                && last_closest_node == closest_node.left_node.as_ref().unwrap().as_ref()
            {
                to_return = SnailfishNumberNode {
                    left_node: Some(Box::new(to_return)),
                    left_number: None,
                    right_node: closest_node.right_node.clone(),
                    right_number: closest_node.right_number.clone(),
                    id: rand::thread_rng().gen_range(0..u32::MAX),
                };

                if debug {
                    println!("Copying node to the left. New node is {}", to_return);
                }

                last_closest_node = closest_node;
                last_copy = false;

                if closest_node.right_node.is_some() {
                    match explosion_state {
                        SnailfishNumberExplosionState::ExplodedLeft => {
                            to_return = SnailfishNumberNode {
                                left_node: to_return.left_node.clone(),
                                left_number: None,
                                right_node: Some(Box::new(
                                    closest_node
                                        .right_node
                                        .as_ref()
                                        .unwrap()
                                        .as_ref()
                                        .add_leftmost(node_to_explode.right_number.unwrap())
                                        .1,
                                )),
                                right_number: None,
                                id: rand::thread_rng().gen_range(0..u32::MAX),
                            };

                            explosion_state = SnailfishNumberExplosionState::Done;
                        }
                        _ => (),
                    }
                }

                continue;
            }

            if closest_node.right_node.is_some()
                && last_closest_node == closest_node.right_node.as_ref().unwrap().as_ref()
            {
                //println!("here");
                //println!("{}", to_return);
                to_return = SnailfishNumberNode {
                    left_node: closest_node.left_node.clone(),
                    left_number: closest_node.left_number.clone(),
                    right_node: Some(Box::new(to_return)),
                    right_number: None,
                    id: rand::thread_rng().gen_range(0..u32::MAX),
                };

                if debug {
                    println!("Copying node to the right. New node is {}", to_return);
                }

                last_closest_node = closest_node;
                last_copy = true;

                if closest_node.left_node.is_some() {
                    match explosion_state {
                        SnailfishNumberExplosionState::ExplodedRight => {
                            to_return = SnailfishNumberNode {
                                left_node: Some(Box::new(
                                    closest_node
                                        .left_node
                                        .as_ref()
                                        .unwrap()
                                        .as_ref()
                                        .add_rightmost(node_to_explode.left_number.unwrap())
                                        .1,
                                )),
                                left_number: None,
                                right_node: to_return.right_node.clone(),
                                right_number: None,
                                id: rand::thread_rng().gen_range(0..u32::MAX),
                            };

                            /*println!("Yo");
                            println!("{:?}", to_return.left_node.as_ref().unwrap().as_ref());
                            println!("{}", to_return.right_node.as_ref().unwrap().as_ref());*/

                            explosion_state = SnailfishNumberExplosionState::Done;

                            if debug {
                                println!("Copied node to the right and also found a left node while in {:?}. New node is {}", SnailfishNumberExplosionState::ExplodedRight, to_return);
                            }
                        }
                        _ => (),
                    }
                }

                continue;
            }

            to_return = closest_node.clone();
        }

        match explosion_state {
            SnailfishNumberExplosionState::Done => return to_return,
            SnailfishNumberExplosionState::ExplodedLeft => {
                if !last_copy && to_return.right_node.is_none() {
                    return to_return;
                }

                if last_copy {
                    return to_return;
                }

                return SnailfishNumberNode {
                    left_node: to_return.left_node.clone(),
                    left_number: to_return.left_number.clone(),
                    right_node: Some(Box::new(
                        to_return
                            .right_node
                            .unwrap()
                            .as_ref()
                            .add_leftmost(node_to_explode.right_number.unwrap())
                            .1,
                    )),
                    right_number: None,
                    id: rand::thread_rng().gen_range(0..u32::MAX),
                };
            }
            SnailfishNumberExplosionState::ExplodedRight => {
                if last_copy && to_return.left_node.is_none() {
                    return to_return;
                }

                if !last_copy {
                    return to_return;
                }

                return SnailfishNumberNode {
                    left_node: Some(Box::new(
                        to_return
                            .left_node
                            .unwrap()
                            .as_ref()
                            .add_rightmost(node_to_explode.left_number.unwrap())
                            .1,
                    )),
                    left_number: None,
                    right_node: to_return.right_node.clone(),
                    right_number: to_return.right_number.clone(),
                    id: rand::thread_rng().gen_range(0..u32::MAX),
                };
            }
            _ => unreachable!(),
        }
    }

    fn add_leftmost(&self, val: u32) -> (bool, SnailfishNumberNode) {
        if self.left_number.is_some() {
            return (
                true,
                SnailfishNumberNode {
                    left_number: Some(self.left_number.unwrap() + val),
                    left_node: None,
                    right_number: self.right_number,
                    right_node: self.right_node.clone(),
                    id: rand::thread_rng().gen_range(0..u32::MAX),
                },
            );
        }

        if self.left_node.is_some() {
            let modified = self.left_node.as_ref().unwrap().as_ref().add_leftmost(val);

            if modified.0 {
                return (
                    true,
                    SnailfishNumberNode {
                        left_number: None,
                        left_node: Some(Box::new(modified.1)),
                        right_number: self.right_number,
                        right_node: self.right_node.clone(),
                        id: rand::thread_rng().gen_range(0..u32::MAX),
                    },
                );
            }
        }

        return (false, self.clone());
    }

    fn add_rightmost(&self, val: u32) -> (bool, SnailfishNumberNode) {
        if self.right_number.is_some() {
            return (
                true,
                SnailfishNumberNode {
                    left_number: self.left_number,
                    left_node: self.left_node.clone(),
                    right_number: Some(self.right_number.unwrap() + val),
                    right_node: None,
                    id: rand::thread_rng().gen_range(0..u32::MAX),
                },
            );
        }

        if self.right_node.is_some() {
            let modified = self
                .right_node
                .as_ref()
                .unwrap()
                .as_ref()
                .add_rightmost(val);

            if modified.0 {
                return (
                    true,
                    SnailfishNumberNode {
                        left_number: self.left_number,
                        left_node: self.left_node.clone(),
                        right_number: None,
                        right_node: Some(Box::new(modified.1)),
                        id: rand::thread_rng().gen_range(0..u32::MAX),
                    },
                );
            }
        }

        return (false, self.clone());
    }

    fn split(&self) -> (bool, SnailfishNumberNode) {
        if self.left_number.is_some() && self.left_number.unwrap() > 9 {
            let left_num = (f64::from(self.left_number.unwrap()) / f64::from(2)).floor();
            let right_num = (f64::from(self.left_number.unwrap()) / f64::from(2)).ceil();

            let inner = SnailfishNumberNode {
                left_number: Some(left_num as u32),
                right_number: Some(right_num as u32),
                left_node: None,
                right_node: None,
                id: rand::thread_rng().gen_range(0..u32::MAX),
            };

            return (
                true,
                SnailfishNumberNode {
                    left_node: Some(Box::new(inner)),
                    left_number: None,
                    right_number: self.right_number,
                    right_node: self.right_node.clone(),
                    id: rand::thread_rng().gen_range(0..u32::MAX),
                },
            );
        }

        if self.left_node.is_some() {
            let (split, new_node) = self.left_node.as_ref().unwrap().as_ref().split();

            if split {
                return (
                    true,
                    SnailfishNumberNode {
                        left_node: Some(Box::new(new_node)),
                        left_number: None,
                        right_number: self.right_number,
                        right_node: self.right_node.clone(),
                        id: rand::thread_rng().gen_range(0..u32::MAX),
                    },
                );
            }
        }

        if self.right_number.is_some() && self.right_number.unwrap() > 9 {
            let left_num = (f64::from(self.right_number.unwrap()) / f64::from(2)).floor();
            let right_num = (f64::from(self.right_number.unwrap()) / f64::from(2)).ceil();

            let inner = SnailfishNumberNode {
                left_number: Some(left_num as u32),
                right_number: Some(right_num as u32),
                left_node: None,
                right_node: None,
                id: rand::thread_rng().gen_range(0..u32::MAX),
            };

            return (
                true,
                SnailfishNumberNode {
                    left_node: self.left_node.clone(),
                    left_number: self.left_number,
                    right_number: None,
                    right_node: Some(Box::new(inner)),
                    id: rand::thread_rng().gen_range(0..u32::MAX),
                },
            );
        }

        if self.right_node.is_some() {
            let (split, new_node) = self.right_node.as_ref().unwrap().as_ref().split();

            if split {
                return (
                    true,
                    SnailfishNumberNode {
                        left_node: self.left_node.clone(),
                        left_number: self.left_number,
                        right_number: None,
                        right_node: Some(Box::new(new_node)),
                        id: rand::thread_rng().gen_range(0..u32::MAX),
                    },
                );
            }
        }

        return (false, self.clone());
    }

    fn reduce(&self, debug: bool) -> SnailfishNumberNode {
        let mut to_return = self.clone();

        loop {
            if debug {
                println!("{}", to_return);
            }
            let (modified, new_node) = to_return.explode(&mut 0, &mut vec![], debug);

            if modified {
                //println!("yes1");
                to_return = new_node;
                continue;
            }

            let (modified, new_node) = to_return.split();

            if modified {
                //println!("yes2");
                to_return = new_node;
                continue;
            }

            break;
        }

        to_return
    }

    fn magnitude(&self) -> u32 {
        let mut mag = 0_u32;

        if self.left_number.is_some() {
            mag += self.left_number.unwrap() * 3;
        }

        if self.left_node.is_some() {
            mag += self.left_node.as_ref().unwrap().as_ref().magnitude() * 3;
        }

        if self.right_number.is_some() {
            mag += self.right_number.unwrap() * 2;
        }

        if self.right_node.is_some() {
            mag += self.right_node.as_ref().unwrap().as_ref().magnitude() * 2;
        }

        mag
    }
}

impl Display for SnailfishNumberNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}
