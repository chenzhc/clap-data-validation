
fn main() {
    // age_validator();

    // type_validator();

    first_name();

}

fn first_name() {
    let first_name_arg = clap::Arg::new("first-name")
        .long("first-name")
        .help("Please enter your first name with a capital letter.")
        .value_parser(first_name_validator);

    let cmd = clap::Command::new("base")
        .arg(first_name_arg);

    let result = cmd.get_matches();
    
    println!("The first name you entered was {0}",
        result.get_one::<String>("first-name").unwrap());

}

fn first_name_validator(value: &str) -> Result<String, std::io::Error> {

    let first_name_regex = regex::Regex::new("[A-Z]\\w+")
        .expect("You wrote an invalid regex. Please fix it.");

    if first_name_regex.is_match(value) {
        return Ok(value.to_string())
    }


    return Err(std::io::Error::new(std::io::ErrorKind::Other,
         "Please make sure first name is capitalied"));
}



fn type_validator() {
    let coat_types = ["winter", "rain", "hoodie"];

    let coat_arg = clap::Arg::new("coat-type")
        .long("coat-type")
        .short('c')
        .help("This coat type validated")
        .value_parser(coat_types);

    let cmd = clap::Command::new("coatType")
        .arg(coat_arg);

    let result = cmd.get_matches();

    println!("This coat type you requested was {0}",
        result.get_one::<String>("coat-type").unwrap());

}

fn age_validator() {
    let age_arg = clap::Arg::new("age")
        .long("age")
        .short('a')
        .help("This is the age that will be validated against.")
        .value_parser(clap::value_parser!(u8).range(25..=40));

    let cmd = clap::Command::new("base")
        .arg(age_arg);
    
    let result = cmd.get_matches();

    println!("This age you passed in was {0}", 
        result.get_one::<u8>("age").unwrap());

}