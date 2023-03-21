use std::fs;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let expression = fs::read_to_string("expression.txt").expect("Unable to read the file");
    println!("Expression: {expression}");

    let query = Regex::new(r"\$\[(?P<cols>\w*[,\w*]*)\]\(@\[(?P<theta>[a-zA-Z0-9<>=&\| ]*)\]\((?P<table>.*)\)\)").unwrap();
    let cap = query.captures(&expression).unwrap();
    let cols = &cap["cols"];
    let theta = &cap["theta"];
    let table = &cap["table"];
    println!("{theta} {table} {cols}");

    let compound = query.replace_all(&expression, "$theta");
    let mut conditions = compound.split(" ").map(|x| x.trim()).collect::<Vec<_>>();
    conditions.retain(|x| x != &"&&" && x != &"||" );

    let columns = cols.split(',').map(|x| x.trim()).collect::<Vec<_>>();

    let mut counter = 1;
    for condition in conditions.iter().permutations(conditions.len()) {
        for column in columns.iter().permutations(columns.len()) {
            let column_string = column.iter().join(", ");
            let condition_string = condition.iter().join(" and ");

            println!("{:2}: $[{}](@[{}]({}))", counter, column_string, condition_string, table);

            counter += 1;
        }
    }
}
