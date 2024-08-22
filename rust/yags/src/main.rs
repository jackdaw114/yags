use clap::Parser;

#[derive(Debug)]
#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
    
}

struct CustomError(String);

impl std::fmt::Debug for CustomError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
}
}


fn main() -> Result<(),CustomError> {
    let args = Cli::parse();

//    let content = std::fs::read_to_string(path).map_err(|error| CustomError(format!("Error reading `{}` : {}",path,error)))?;

    println!("args struct {:?}",args);
    Ok(())
}
