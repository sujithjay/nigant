// Copyright 2020 Sujith Jay Nair
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::parser::Entry;
use crate::parser::Payload;
use crate::parser::Sense;

extern crate termion;
use termion::{color, style};

pub fn print_not_found(query: &String) {
    println!("{}I could not find {}{}{} in my thesauri.{}",
        color::Fg(color::LightRed),
        color::Fg(color::Red), 
        query,
        color::Fg(color::LightRed), 
        color::Fg(color::Reset));
}

pub fn print(payload: &Payload, query: &String) {

    let mut defn: Vec<&String> = Vec::new();
    let mut synm: Vec<&String> = Vec::new();
    let mut etym: Vec<&String> = Vec::new();

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

        let mut definitions: Vec<&String> = senses
            .iter()
            .flat_map(|n| &n.definitions)
            .collect();

        let mut synonyms: Vec<&String> = senses
            .iter()
            .flat_map(|o| o.synonyms.iter().map( |s| &s.text))
            .collect();

        let mut etymologies: Vec<&String> = entries
            .iter()
            .flat_map(|m| &m.etymologies)
            .collect();

        defn.append(&mut definitions);
        etym.append(&mut etymologies);
        synm.append(&mut synonyms);
    }

    println!("");
    println!("{}{}{}", color::Fg(color::Cyan), query, color::Fg(color::Reset));
    println!("{}{}Definitions:{}", style::Italic, color::Fg(color::Yellow), style::Reset);

    for d in &defn {
        println!("\t - {}", d)
    }
    println!("");

    if !synm.is_empty(){
        println!("{}{}Synonyms:{}",style::Italic,color::Fg(color::Yellow), style::Reset);
        let syn: Vec<String> = synm
            .iter()
            .map(|s| format!("{}", s))
            .collect();
        println!("{}", syn.join(", "));
        println!("");
    }

    if !etym.is_empty(){
        println!("{}{}Etymology:{}",style::Italic,color::Fg(color::Yellow), style::Reset);
        for e in &etym {
            println!("\t - {}", e)
        }
        println!("");
    }
}
