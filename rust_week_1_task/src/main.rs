fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::from("");
    result.push_str(s1);
    result.push_str(s2);
    result

}

fn main(){
    let string1 = String::from("the first string");
    let string2 = String::from(" and the second string");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);

}