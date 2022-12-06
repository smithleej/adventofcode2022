use std::collections::VecDeque;

fn main() {
    let mut chars = include_str!("input.txt").chars();

    let mut buffer = VecDeque::from([chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap()]);

    let mut pos = 4;

    while let Some(next_char) = chars.next() {
        if are_unque(Vec::from(buffer.clone())) {
            println!("{:?} pos in {}", buffer, pos);
            break;
        } else {  
            pos = pos + 1;
            buffer.pop_front();
            buffer.push_back(next_char);
        }
    }
}

fn are_unque(buffer: Vec<char>) -> bool {
    let mut count = 0;
    let mut unique = true;

    println!("Checking {:?}", buffer);

    for ele1 in buffer.clone() {
        for ele2 in buffer.clone() {
            if ele1 == ele2 {
                count = count + 1;
            }
        }
        if count > 1 {
            println!("not unique found {} {} times", ele1, count);
            unique = false;
            break;
        } else {
            println!("unique found {} {} times", ele1, count);
            count = 0;
        }
    }
    println!("unique {:?}", unique);
    unique
}
