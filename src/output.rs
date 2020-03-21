use crate::parser::Entry;
use crate::parser::Payload;
use crate::parser::Sense;

extern crate termion;
use termion::{color, style};

pub fn print(payload: &Payload, query: String) {

    println!("{}{}{}", color::Fg(color::Cyan), query, color::Fg(color::Reset));
    println!("{}{}Definitions:{}", style::Italic, color::Fg(color::Yellow), style::Reset);

    for word in &payload.results {
        let entries: Vec<&Entry> = word
            .lexical_entries
            .iter()
            .flat_map(|l| &l.entries )
            .collect();

        let senses: Vec<&Sense> = entries
            .iter()
            .flat_map(|m| &m.senses)
            .collect();

        let definitions: Vec<&String> = senses
            .iter()
            .flat_map(|n| &n.definitions)
            .collect();

        let synonyms: Vec<&String> = senses
            .iter()
            .flat_map(|o| o.synonyms.iter().map( |s| &s.text))
            .collect();

        for defn in &definitions {
            println!("\t - {}", defn)
        }
        if !synonyms.is_empty(){
            println!("");
            println!("{}{}Synonyms:{}",style::Italic,color::Fg(color::Yellow), style::Reset);
            let syn: Vec<String> = synonyms
                .iter()
                .map(|s| format!("{}", s))
                .collect();
            println!("{}", syn.join(", "));
            println!("");
        }
    }
}