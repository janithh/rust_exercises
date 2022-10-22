mod case_insensitive_sort;

fn main() {
    let mut dataset = vec![String::from("satya1981"), String::from("Tom"), String::from("satya_99"), String::from("Satyajith"), String::from("amy"), String::from("satya99"), String::from("MAhesh"), String::from("maheel")]; 

    println!("[case_insensitive_sort::main] User Names - {:?}", dataset);
    case_insensitive_sort::exercise::sort_usernames(&mut dataset);
    println!("[case_insensitive_sort::main] User Names - {:?}", dataset);
}