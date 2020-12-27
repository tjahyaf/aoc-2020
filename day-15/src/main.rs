use std::collections::HashMap;

fn main() {
    let input : Vec<usize> = "0,1,5,10,3,12,19".split(",").map(|n| n.parse()).flatten().collect();

    let num = game(&input, 30_000_000);
    println!("{}", num);
}

fn game(input: &[usize], turns: usize) -> usize {
    let mut numbers : HashMap<usize, (usize, usize)> = HashMap::new();
    let mut last = input.last().unwrap().to_owned();
    
    for (i, num) in input.iter().enumerate() {
        numbers.insert(*num, (i+1, 0));
    }

    for turn in input.len()+1..=turns {
        let (prev, earlier) = numbers.get(&last).or(Some(&(0usize, 0usize))).unwrap().to_owned();
        
        if earlier == 0 {
            last = 0;
        } else {
            last = prev - earlier;
        }

        let (prev, _) = numbers.get(&last).or(Some(&(0usize, 0usize))).unwrap().to_owned();
        numbers.insert(last, (turn, prev));
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        println!("{}", game(&[0, 3, 6], 2020));
    }
}