use std::str::FromStr;

pub fn get<T: FromStr>(args: &[String], name: &str) -> Option<T> {
    let name = name.to_lowercase();
    args.iter()
        .position(|arg| *arg.to_lowercase() == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|arg| arg.parse::<T>().ok())
}

pub fn some(args: &[String], name: &str) -> bool {
    let name = name.to_lowercase();
    args.iter().any(|arg| *arg.to_lowercase() == name)
}
