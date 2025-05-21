/*
Project Name: Temperature Converter
Author: Matthew Gasiorek
Created On: 2025-05-19
Last Modified: 2025-05-19
Description: I want this to be a fun, introductory project where I can learn the basics of Rust. Rust seems like a cool language that is C-adjacent.
Version: 1.0
Contact: matthew.gasiorek.coding@gmail.com
*/

//std is the rust standard library and io is for core i/o functionality (read and write)
use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");
    println!("Please enter the temperature value!");
    
    //The primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.
    //mut means mutable, and can be in reference to a variable, reference, or pointer
    //also, mutable means it's able to be changed at any point in the program
    let mut temp_input = String::new();

    //Read the user's input from the keyboard and put it into temp_input.
    //io::stdin() gives access to the standard input stream, the keyboard
    //.read_line(&mut temp_input) basically tells the program to read whatever the user types and then store the text into temp_input by using mut
    io::stdin().read_line(&mut temp_input)
    //'.expect()' is a basic way to handle potential errors. If reading fails, the program will crash and print the message.
        .expect("Failed to read line. Something went wrong with input."); //this might not be needed

    //we declared a new variable here said we wanted it to be a floating piont number (f64) and it will be the whatever is in temp_input. We also set it up so it trims to remove any extra spaces. parse() basically converts the string to another type. In this case, into a floating point value. match is like a switch statement in excel.
    let temperature: f64 = match temp_input.trim().parse()
    {
        Ok(num) => num, //If parsing is successful, num contains the number. 
        Err(_) => { //if parsing is unsuccessful, Err indicates an error. The symbol "_" is a catch-all.
            println!("Invalid temperature entered. Please enter a valid number.");
            return; //the program stops here if we don't get a valid number.
        }
    };

    //now we have to enter the unit of temperature. same as we did for temp_input, we use the let mut... line
    //and we also use io::stdin().read_line as before
    println!("Enter the unit of temperature (C for Celsius, F for Fahrenheit):");
    let mut unit_input = String::new();
    io::stdin().read_line(&mut unit_input)
        .expect("Failed to read unit input. Something went wrong.");
    
    let unit = unit_input.trim().to_uppercase(); //we want to use .to_uppercase() here to make it so if they enter c, it turns into C and f into F.

    let converted_temp: f64;
    let converted_unit_name: &str;

    if unit == "C" {
        converted_temp = celsius_to_fahrenheit(temperature);
        converted_unit_name = "Fahrenheit";
    }

    else if unit == "F" {
        converted_temp = fahrenheit_to_celsius(temperature);
        converted_unit_name = "Celsius";
    }

    else {
        println!("Invalid unit entered. Please use C for Celsius or F for Fahrenheit.");
        return;
    }
    //printing the final result
    println!("\n{:.2} degrees {} is {:.2} degrees {}.", temperature, if unit == "C" {"Celsius"} else {"Fahrenhiet"}, converted_temp, converted_unit_name);
}

//outside of the main function, we need to create functions that convert between celsius and fahrenheit

//this one converts celsius to fahrenheit. once we give the function a name, we give it the variables it works with and what type of input to expect (in this case a floating point value with up to 64 decimals). then the -> means that it returns a specific type (in this case a floating point value with up to 64 decimals). The actual function of the function is then put in curly braces.
fn celsius_to_fahrenheit(celsius: f64) -> f64 {(celsius * 9.0 / 5.0) + 32.0}

//this one is for fahrenheit to celsius conversions.
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {(fahrenheit - 32.0) * 5.0 / 9.0}