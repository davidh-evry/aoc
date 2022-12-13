use std::cmp::Ordering;

fn main() {
    let pairs = parse_pairs("res/day13.txt");
    let sum = pairs
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| *l < *r)
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("{sum}");
    let mut elements = pairs
        .into_iter()
        .flat_map(|(l, r)| [l, r].into_iter())
        .collect::<Vec<_>>();
    let two = parse_line("[[2]]").1;
    let six = parse_line("[[6]]").1;
    elements.push(two.clone());
    elements.push(six.clone());
    elements.sort();
    let prod = elements
        .into_iter()
        .enumerate()
        .filter(|(_, e)| *e == two || *e == six)
        .map(|(i, _)| i + 1)
        .product::<usize>();
    println!("{prod}");
}

fn parse_pairs(path: &str) -> Vec<(Element, Element)> {
    let file_content = std::fs::read_to_string(path).unwrap();
    let lines = file_content.lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .map(|chunk| (parse_line(&chunk[0]).1, parse_line(&chunk[1]).1))
        .collect()
}

fn parse_line(line: &str) -> (usize, Element) {
    let mut current_element = vec![];
    let chars = line.chars().collect::<Vec<_>>();
    let mut i = 1;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '[' => {
                let (count, element) = parse_line(&line[i..]);
                i += count;
                current_element.push(element)
            }
            ']' => return (i, Element::List(current_element)),
            '0'..='9' => {
                let string_num = chars[i..]
                    .iter()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>();
                i += string_num.len() - 1;
                current_element.push(Element::Int(string_num.parse().unwrap()));
            }
            ',' => (),
            _ => panic!("Unknown character"),
        }
        i += 1;
    }
    (i, Element::List(current_element))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    List(Vec<Element>),
    Int(u32),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Element::List(items1) => match other {
                Element::List(items2) => {
                    for i in 0..items1.len() {
                        if i == items2.len() {
                            return Ordering::Greater;
                        }
                        let cmp = items1[i].cmp(&items2[i]);
                        if cmp != Ordering::Equal {
                            return cmp;
                        }
                    }
                    if items1.len() < items2.len() {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
                Element::Int(_) => self.cmp(&Element::List(vec![other.clone()])),
            },
            Element::Int(i1) => match other {
                Element::List(_) => Element::List(vec![self.clone()]).cmp(other),
                Element::Int(i2) => i1.cmp(i2),
            },
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
