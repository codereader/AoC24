fn main() {
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();

/* 
// Create an empty mutable string
let mut file_content = "3   4
4   3
2   5
1   3
3   9
3   3";
*/

    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    let normalized_file = file_content.replace('\r', "");
    let lines = normalized_file.split('\n').filter(|&x| !x.is_empty());
}
