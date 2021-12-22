use std::collections::HashMap;
use std::vec::Vec;

enum TeamplateBlock {
    Text(String),
    Parameter(String),
}

fn simple_teamplate_parse(input: &str, teamplate: Vec<TeamplateBlock>) -> Option<Vec<String>> {
    let input_words: Vec<&str> = input.split_whitespace().collect();
    let mut result: HashMap<String, String> = HashMap::new();

    if input_words.len() != teamplate.len() {
        println!("Input does not match pattern");
        return None
    }

    for n in 0..input_words.len() {
        let word = input_words[n];
        let t_block = teamplate[n];

        match t_block {
            TeamplateBlock::Text(d) => if !word.eq(d) { return None }
            TeamplateBlock::Parameter(d) => result.insert(String::from(d), String::from(word))
        }
    }

    Some(result)
}

pub struct TextInterface {
    bd: HashMap<String, Vec<String>>,
}

impl TextInterface {
    const ADD_TEAMPLATE: Vec<TeamplateBlock> = vec![
        TeamplateBlock::Text("Add"),
        TeamplateBlock::Parameter("emplyee"),
        TeamplateBlock::Text("to"),
        TeamplateBlock::Parameter("department"),
    ];

    const REMOVE_TEAMPLATE: Vec<TeamplateBlock> = vec![
        TeamplateBlock::Text("Remove"),
        TeamplateBlock::Parameter("emplyee"),
        TeamplateBlock::Text("from"),
        TeamplateBlock::Parameter("department"),
    ];

    pub fn new() -> TextInterface {
        TextInterface {
            bd: HashMap::new(),
        }
    }

    pub fn parse(&self, s: &str) {
        if let Some(add_teamplate_result) = simple_teamplate_parse(s, TextInterface::ADD_TEAMPLATE) {
            self.handle_add_teamplate(add_teamplate_result)
        }

        if let Some(remove_teamplate_result) = simple_teamplate_parse(s, TextInterface::REMOVE_TEAMPLATE) {
            self.handle_remove_teamplate(remove_teamplate_result)
        }
    }

    pub fn print_db(&self) {

    }

    fn handle_add_teamplate(&self, paramters: HashMap<String, String>) {
        self.add(paramters.get("employee"), paramters.get("department"));
    }

    fn handle_remove_teamplate(&self, paramters: HashMap<String, String>) {
        self.remove(paramters.get("employee"), paramters.get("department"));
    }

    fn add(&self, employee: &str, department: &str) {
        if let Some(employes) = self.bd.get(department) {
            employes.push(employee);
        } else {
            let employes: Vec<String> = Vec::new();
            employes.push(employee);
            self.bd.insert(department, employes);
        }
    }

    fn remove(&self, employee: &str, department: &str) {
        if let Some(employes) = self.bd.get(department) {
            employes.push(employee);
        } else {
            let employes: Vec<String> = Vec::new();
            employes.push(employee);
            self.bd.insert(department, employes);
        }
    }
}