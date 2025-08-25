pub fn search<'a>(query: &str,contents: &'a str)-> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    if results.is_empty() {
        println!("Err:: coudn't find the line containing \"{}\" ",query);
        results
    }
    else {
        print!("Ok:: found the line---> ");
        results
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query, contents));

    }
}