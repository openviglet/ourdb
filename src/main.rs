pub mod version;
fn main() {
    let mut version_string: String = "Current release: ".to_owned();
    let current_version: String = version::get_version("").to_owned();
    version_string.push_str(&current_version);
    println!("{}", version_string);

}
