extern crate clap;
use clap::{Arg, App};
use std::process::Command;


fn main() {
    let matches = App::new("mypipe")
                          .version("1.0")
                          .author("Elif CILINGIR <elif.cilingir061@gmail.com>")
                          .about("pipe")
                          .arg(Arg::with_name("input")
                               .short("in")
                               .long("in")
                               .value_name("intput")
                               .help("Sets the input to use")
                               .takes_value(true))
                            .arg(Arg::with_name("output")
                               .short("out")
                               .long("out")
                               .value_name("output")
                               .help("Sets the output to use")
                               .takes_value(true))
                          .get_matches();

                          let inputValue = matches.value_of("input").unwrap( );//recupération des arguments après notre input
                          println!("input : {}", inputValue); //affichage de la valeur entrer pour l'input

                          
        

    let outputValue = matches.value_of("output").unwrap( );//recupération des arguments après notre output
    println!("input : {}", outputValue); //affichage de la valeur entrer pour l'output

    
    let inputExecution = Command::new(inputValue.to_string()).output().expect("Error when it try to execute the input");
       
              
    let inputValueArgs = String::from_utf8_lossy(&inputExecution.stdout).to_string();
    
    let outputExecution = Command::new(outputValue.to_string()).arg(inputValueArgs)
                        .output().expect("Error when it try to execute the output");

                         
    let result = String::from_utf8_lossy(&outputExecution.stdout).to_string();

    println!("{}", result );
    

}
