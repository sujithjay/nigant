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

use serde::{Deserialize, Serialize};

extern crate serde_json;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tuple{
    #[serde(default)]
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pronounciation{
    #[serde(default)]
    pub audio_file : String,
    pub dialects: Vec<String>,
    pub phonetic_notation: String,
    pub phonetic_spelling: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example{
    pub text: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThesaurusLink{
    #[serde(default)]
    pub entry_id: String,
    #[serde(default)]
    pub sense_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Synonym{
    #[serde(default)]
    pub id: String,
    pub language: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subsense{
    #[serde(default)]
    pub definitions: Vec<String>,
    #[serde(default)]
    pub domains: Vec<Tuple>,
    pub id: String,
    #[serde(default)]
    pub registers: Vec<Tuple>,
    pub short_definitions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sense{
    #[serde(default)]
    pub definitions: Vec<String>,
    #[serde(default)]
    pub examples: Vec<Example>,
    pub id: String,
    #[serde(default)]
    pub short_definitions: Vec<String>,
    #[serde(default)]
    pub subsenses: Vec<Subsense>,
    #[serde(default)]
    pub synonyms: Vec<Synonym>,
    #[serde(default)]
    pub thesaurus_links: Vec<ThesaurusLink>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry{
    #[serde(default)]
    pub etymologies: Vec<String>,
    pub senses: Vec<Sense>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LexicalEntry{
    #[serde(default)]
    pub derivatives: Vec<Tuple>,
    pub entries: Vec<Entry>,
    pub language: String,
    pub lexical_category: Tuple,
    pub pronunciations:Vec<Pronounciation>,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    #[serde(default)]
    pub id: String,
    pub language: String,
    pub lexical_entries: Vec<LexicalEntry>,
    pub r#type : String,
    pub word: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(default)]
    pub operation: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub schema: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    #[serde(default)]
    pub id : String,
    #[serde(default= "default_metadata")]
    pub metadata: Metadata,
    pub results:Vec<Word>,
}

fn default_metadata() -> Metadata {
    return Metadata {
        operation: String::from(""), 
        provider: String::from(""), 
        schema: String::from("")
    };
}

pub fn parse(json: &String) -> Payload {
    serde_json::from_str(json).unwrap()
}