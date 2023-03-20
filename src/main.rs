use std::{fs, io::BufRead};
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
    // let conditions = Regex::new(r"([a-zA-Z0-9_<>=!]+)( (and|or) ([a-zA-Z0-9_<>=!]+))*").unwrap();

    let compound = query.replace_all(&expression, "$theta");
    let mut conditions = compound.split(" ").map(|x| x.trim()).collect::<Vec<_>>();
    conditions.retain(|x| x != &"&&" && x != &"||" );

    let columns = cols.split(',').map(|x| x.trim()).collect::<Vec<_>>();

    // let conds: Vec<String> = conditions.captures_iter(&after).map(|x| String::from(&x[0])).collect::<Vec<String>>();
    // println!("{:?}", conds);

    for condition in conditions.iter().permutations(conditions.len()) {
        for column in columns.iter().permutations(columns.len()) {
            // println!("{:?} {:?} {:?}", column, condition, table);
            let column_string = column.iter().join(", ");
            let condition_string = condition.iter().join(" and ");
            println!("$[{}](@[{}]({}))", column_string, condition_string, table);
        }
    }
}
