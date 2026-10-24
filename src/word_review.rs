// umm there might be no need for this file, but ill still make it because why not

use ratatui::{
    Frame,
    layout::Constraint,
    style::Style,
    widdgets::{Block, Cell, Row, Table},
};

pub fn find_mistakes(passage: &str, input: &str) -> Vec<(String, String)>{
    let expected: Vec<&str> = passage.split_whitespace().collect();
    let typed: Vec<&str> = input.split_whitespace().collect();

    let mut mistakes = Vec::new();

    for (index, actual) in typed.iter().enumerate(){
        if let Some(target) = expected.get(index){
            if *target != *actual {
                mistakes.push((
                    target.to_string(),
                    actual.to_string(),
                ));
            }
        }
    }
    mistakes
}

pub fn draw(frame: &mut Frame, theme: &crate::theme::Theme, passage: &str, input: &str,){
  
    let mut mistakes = find_mistakes(passage, input);
    let mut rows = Vec::new();

    for (expected, typed) in mistakes {
        // mehhh, this makes the column that shows all the words you spelt wrong blehhhhh
        rows.push(Row::new([
            Cell::from(expected),
            
        ]))
    }
        
    

