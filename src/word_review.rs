// umm there might be no need for this file, but ill still make it because why not

use ratatui::{
    Frame,
    layout::Constraint,
    style::Style,
    widgets::{Block, Cell, Row, Table, TableState},
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

// so this would take all the words that you got wrong and place them in a new page when you would be able to re-type them. lowk this aint even importnt
pub fn practice_words(passage: &str, input: &str) -> Vec<String>{
    let mistakes = find_mistakes(passage, input);
    let mut words = Vec::new();

    for (expected, _) in mistakes {
        if !words.contains(&expected){
            words.push(expected);
        }
    }
    words
}
pub fn draw(frame: &mut Frame, theme: &crate::config::structs::theme::Theme, passage: &str, input: &str, state: &mut TableState,){
  
    let mistakes = find_mistakes(passage, input);
    let total = mistakes.len();
    let position = state.selected().map_or(0, |index| index + 1);
    let mut rows = Vec::new();

    for (expected, typed) in mistakes {
        // mehhh, this makes the column that shows all the words you spelt wrong blehhhhh
        rows.push(Row::new([
            Cell::from(expected),
            Cell::from(typed)
                .style(Style::default().fg(theme.incorrect)),
        ]));
    }

if rows.is_empty(){
    rows.push(Row::new(["lowk no mistakes", ""]));
}

let widths = [
    Constraint::Percentage(50),
    Constraint::Percentage(50),
];

let widget = Table::new(rows, widths)
    .header(Row::new(["Expected", "You typed"]).bottom_margin(1))
    .column_spacing(2)
    .highlight_symbol("> ")
    .style(Style::default().fg(theme.text).bg(theme.background))
    .block(
        Block::bordered()
            .border_style(Style::default().fg(theme.border))
                    .title(format!("tuipe | {position}/{total} mistake | P: practise | Up/Down: scroll | Esc: results"))
            );
            frame.render_stateful_widget(widget, frame.area(), state);
}
