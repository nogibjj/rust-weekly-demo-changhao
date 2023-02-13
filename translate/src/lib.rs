/* A library that uses Hugging Face to translate text */
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::fs::File;
use std::io::Read;

// build a function that reads a file and returns a string  
pub fn read_file(path: String) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// build a function that reads a file and returns array of the lines of the file    
pub fn read_file_lines(path: String) -> anyhow::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

// build a function that reads a file and translates it
pub fn translate_file(path: String) -> anyhow::Result<()> {
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::French])
        .create_model()?;
    let text = read_file_lines(path)?;
    //pass in the text to the model
    let output = model.translate(&text, None, Language::French)?;
    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}





