/*
Project Name: Temperature Converter
Author: Matthew Gasiorek
Created On: 2025-05-19
Last Modified: 2025-05-20
Description: This is a basic temperature converter that converts celsius to fahrenheit and fahrenheit to celsius.
Version: 1.1
Contact: matthew.gasiorek.coding@gmail.com
*/

//std is the rust standard library and io is for core i/o functionality (read and write)
use std::io::{self};

fn main() {
    println!("Welcome to the Temperature Converter!");

    'main_loop: loop {
        //main loop
        println!(
            "Type 'exit' or 'quit', then press the enter key to exit the program, or press the enter key to convert a temperature."
        );
        let mut decision = String::new();
        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to read line. Something went wrong.");

        let decision = decision.trim().to_lowercase();

        if decision == "quit" || decision == "exit" {
            println!("Closing Temperature Converter");
            break 'main_loop;
        }

        //The primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.
        //mut means mutable, and can be in reference to a variable, reference, or pointer
        //also, mutable means it's able to be changed at any point in the program
        let temperature: f64 = loop {
            //loop{} is how we create loops
            println!("Please enter the temperature value:");
            let mut temp_input = String::new();
            io::stdin() //io::stdin() gives access to the standard input stream, the keyboard
                .read_line(&mut temp_input) //.read_line(&mut temp_input) basically tells the program to read whatever the user types and then store the text into temp_input by using mutit into temp_input.
                .expect("Failed to read line. Something went wrong."); //'.expect()' is a basic way to handle potential errors. If reading fails, the program will crash and print the message.

            //we declare a new variable here and we want it to be a floating piont number (f64) and it will be whatever is in temp_input. We also set it up so it trims to remove any extra spaces. parse() basically converts the string to another type. In this case, into a floating point value. match is like a switch statement in excel.
            match temp_input.trim().parse() {
                Ok(num) => break num, // If parsing is successful, break the loop and return the number
                Err(_) => {
                    // If parsing fails, print an error and continue to the next iteration of the loop
                    println!("Invalid temperature entered. Please enter a valid number.");
                    continue; // Go back to the start of the 'loop' block
                }
            };
        };
        let unit: String = loop {
            println!(
                "Please enter the unit of this temperature (C for Celsius, F for Fahrenheit):"
            );
            let mut unit_input = String::new();
            io::stdin()
                .read_line(&mut unit_input)
                .expect("Failed to read line. Something went wrong.");

            let parsed_unit = unit_input.trim().to_uppercase(); // Clean and standardize the input

            // Check if the cleaned unit is valid
            if parsed_unit == "C" || parsed_unit == "F" {
                break parsed_unit; // If valid, break the loop and return the unit string
            } else {
                // If invalid, print an error and continue to the next iteration
                println!("Invalid unit entered. Please use 'C' for Celsius or 'F' for Fahrenheit.");
                continue; // Go back to the start of the 'loop' block
            }
        };

        // --- Performing the conversion based on unit (C or F) ---
        let (converted_temp, converted_unit_name) = if unit == "C" {
            (celsius_to_fahrenheit(temperature), "Fahrenheit")
        } else if unit == "F" {
            (fahrenheit_to_celsius(temperature), "Celsius")
        } else {
            // This 'else' block theoretically shouldn't be reached now due to the loop validation,
            // but it's good practice for exhaustiveness or if future changes bypass the loop.
            // For robustness, we could also make this return an error or panic.
            // For now, let's just panic as it indicates a logical error if we ever reach here.
            panic!("Program logic error: Unit should have been validated by loop!");
        };

        //printing the final result
        println!(
            "\n{:.2} degrees {} is {:.2} degrees {}.",
            temperature,
            if unit == "C" { "Celsius" } else { "Fahrenhiet" },
            converted_temp,
            converted_unit_name
        );
    } // end of main loop
}

//outside of the main function, we need to create functions that convert between celsius and fahrenheit

//this one converts celsius to fahrenheit. once we give the function a name, we give it the variables it works with and what type of input to expect (in this case a floating point value with up to 64 decimals). then the -> means that it returns a specific type (in this case a floating point value with up to 64 decimals). The actual function of the function is then put in curly braces.
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

//this one is for fahrenheit to celsius conversions.
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
