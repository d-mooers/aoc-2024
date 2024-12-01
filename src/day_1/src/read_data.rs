use anyhow::{Ok, Result};


pub fn get_problem(path: &str) -> Result<(Vec<u32>, Vec<u32>)>
{
    let mut vec_1 = vec![];
    let mut vec_2 = vec![];

    let file_contents = std::fs::read_to_string(path).expect("Failed to read input!");

    file_contents.split("\n").for_each(|line| {
        line.split(" ")
        .filter(|s| !s.is_empty())
        .map(|entry| {
            str::parse::<u32>(entry.trim())
        })
        .enumerate()
        .for_each(|(i, val)| {
            let number = val.expect("Unable to parse number");
            if i == 0 {
                vec_1.push(number);
                return;
            }

            vec_2.push(number);
        });
    });

    Ok((vec_1, vec_2))
}