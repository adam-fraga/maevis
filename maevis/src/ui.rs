use crate::request_builder::request_entities::ModelList;
use console::Style;
use console::Term;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn main_menu() {
    let green = Style::new().green().bold();
    println!(
        "{}",
        green.apply_to("Hello and Welcome to Aeris, your AI Assistant 🤖\n")
    );
}

/* HOME MENU */
pub fn user_prefered_request_menu() {
    let green = Style::new().green();
    let blue = Style::new().blue();
    println!(
        "{}",
        green.apply_to("\n Choose a request to execute 🚀  \n")
    );
    println!(
        "{}",
        blue.apply_to(
            "    [1] - Fast answer
    [2] - Specific answer (Choose your prompt)
    [3] - Accurate answer (Choose all params)
    [4] - Translate something
    [5] - Generate an image
    [6] - Complete sentence\n"
        )
    );
}

/* APPEND THIS FUNCTIONNALITY AFTER SETTING THE CUSTOM PROMPT IN JSON FILE in /ai_prompts */

/* CUSTOM PROMPT LIST MENU (Doctor, Software engineer...)*/
/* export function predefined_prompt_menu(ai_prompt_list: Array<AIPrompt) { */
/*   let id = 0; */
/*   console.log( */
/*     chalk.greenBright(" Who do you want me to act as?\n\n") */
/*   ); */
/*   for (aiPrompt in ai_prompt_list) { */
/*     console.log(chalk.blueBright(`   [${id}] - ${aiPrompt}\n\n`)); */
/*     id++; */
/*   } */
/* } */

/* MODEL LIST MENU (if the user want to select one himself*/
pub fn model_list_menu() {
    let green = Style::new().green();
    let blue = Style::new().blue();
    let open_delimitor: char = '[';
    let close_delimitor: char = ']';
    let mut index = 1;

    //Define path to the json file
    let path = Path::new("/Users/adamfraga/Projects/maevis/src/json/open_ai_models.json");
    //Convert json to rust raw string
    let oai_models_data = fs::read_to_string(path).expect("Unable to read file");
    //Deserialise raw string to a rust struct type
    let oai_model: ModelList = serde_json::from_str(&oai_models_data).unwrap();
    println!("{}", green.apply_to("\n Choose your AI model 📚 \n"));
    for model in oai_model.data {
        println!(
            "    {}{}{} {}",
            blue.apply_to(open_delimitor),
            blue.apply_to(index),
            blue.apply_to(close_delimitor),
            blue.apply_to(model.name)
        );
        index += 1;
    }
}

/* TEMPERATURE MENU (if the user want to select one himself*/
pub fn temperature_menu() {
    let green = Style::new().green();
    let blue = Style::new().blue();

    println!(
        "{}",
        green.apply_to("\n Choose a temperature value for your request 🌡️\n")
    );
    println!(
        "{}",
        blue.apply_to("    [1] Low \n    [2] Medium \n    [3] High")
    );
}

/* MAX_TOKEN MENU (if the user want to select one himself*/
pub fn max_token_menu() {
    let green = Style::new().green();
    let blue = Style::new().blue();

    println!(
        "{}",
        green.apply_to("\n Choose the maximum token for your response ⌨️ \n")
    );
    println!(
        "{}",
        blue.apply_to("    [1] Low \n    [2] Medium \n    [3] High")
    );
}

pub fn prompt_menu() {
    let green = Style::new().green();

    println!(
        "{}",
        green.apply_to("\n What is your demand today ? 💬  \n")
    );
}
