use std::fs;

fn main() {
    let file_path = "./src/01/input.txt";
    let file_content = fs::read_to_string(file_path).expect("Input not found");

    let mut total = 0;
    file_content.lines().for_each(|x| {
        total += get_coordinate_from_input(x);
    });

    println!("The total is: {total}");
}

fn get_coordinate_from_input(line: &str) -> i32 {
    let numbers = line.match_indices(char::is_numeric);

    let ones = line.match_indices("one").map(|(idx, _)| (idx, "1"));
    let twos = line.match_indices("two").map(|(idx, _)| (idx, "2"));
    let threes = line.match_indices("three").map(|(idx, _)| (idx, "3"));
    let fours = line.match_indices("four").map(|(idx, _)| (idx, "4"));
    let fives = line.match_indices("five").map(|(idx, _)| (idx, "5"));
    let sixes = line.match_indices("six").map(|(idx, _)| (idx, "6"));
    let sevens = line.match_indices("seven").map(|(idx, _)| (idx, "7"));
    let eights = line.match_indices("eight").map(|(idx, _)| (idx, "8"));
    let nines = line.match_indices("nine").map(|(idx, _)| (idx, "9"));

    let mut collected_numbers: Vec<(usize, &str)> = (numbers
        .chain(ones)
        .chain(twos)
        .chain(threes)
        .chain(fours)
        .chain(fives)
        .chain(sixes)
        .chain(sevens)
        .chain(eights)
        .chain(nines))
    .collect::<Vec<(usize, &str)>>();

    collected_numbers.sort_by(|a, b| a.0.cmp(&b.0));

    if collected_numbers.len() < 1 {
        return 0;
    }

    let start: String = collected_numbers.first().unwrap().1.to_string();
    let end: &str = collected_numbers.last().unwrap().1;

    let output = start + end;

    return output.parse::<i32>().unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_number_string() {
        assert_eq!(get_coordinate_from_input("jcb82eightwond"), 82)
    }

    #[test]
    fn test_only_with_numeric_numbers() {
        assert_eq!(get_coordinate_from_input("13dzbmbtl6"), 16);
    }

    #[test]
    fn test_with_mixed() {
        assert_eq!(get_coordinate_from_input("fourp783fiveseventhree"), 43);
    }
}
