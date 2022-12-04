use dotenv;

fn main(envID: String) {
    let envValue = dotenv::var(envID).unwrap();
    return envValue;
}