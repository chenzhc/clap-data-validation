fn main() {
    // age_validator();

    type_validator();

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