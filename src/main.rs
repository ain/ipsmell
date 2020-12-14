
const API_KEY:&str = "";
const API_URL:&str = "";

fn main() {
    if API_KEY.is_empty() || API_URL.is_empty() {
        panic!("Invalid API credentials!");
    }
}
