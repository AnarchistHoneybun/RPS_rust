use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::Rng;
use ratatui::{prelude::*, widgets::*};

static ROCK: &'static str = "   _______
---'   ____)
      (_____)
      (_____)
      (____)
---._(___)";

static PAPER: &'static str = "______
---'   ____)____
           ______)
          _______)
        _______)
---.________)";

static SCISSORS: &'static str = "______
---'   ____)____
          ______)
       __________)
      (____)
--._(___)";

//function game_prog that takes a mutable reference to the app
fn game_prog(app: &mut App, p1_move: String){
    match p1_move.as_str() {
        "r" => app.p1_move = String::from(ROCK),
        "p" => app.p1_move = String::from(PAPER),
        "s" => app.p1_move = String::from(SCISSORS),
        _ => app.p1_move = String::from(""),
    }

    //generate a random number between 0 and 2
    let mut rng = rand::thread_rng();
    let p2_move = rng.gen_range(0..3);
    match p2_move {
        0 => app.p2_move = String::from(ROCK),
        1 => app.p2_move = String::from(PAPER),
        2 => app.p2_move = String::from(SCISSORS),
        _ => app.p2_move = String::from(""),
    }

    //determine the winner
    if app.p1_move == app.p2_move {
        //tie
    }
    else if app.p1_move == String::from(ROCK) && app.p2_move == String::from(SCISSORS) {
        app.p1_score += 1;
    }
    else if app.p1_move == String::from(PAPER) && app.p2_move == String::from(ROCK) {
        app.p1_score += 1;
    }
    else if app.p1_move == String::from(SCISSORS) && app.p2_move == String::from(PAPER) {
        app.p1_score += 1;
    }
    else {
        app.p2_score += 1;
    }
}
struct App {
    //player 1 move string
    p1_move: String,
    //player 2 move string
    p2_move: String,
    //player 1 score
    p1_score: u32,
    //player 2 score
    p2_score: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Create a new app
    let mut app = App {
        p1_move: String::from(""),
        p2_move: String::from(""),
        p1_score: 0,
        p2_score: 0,
    };

    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|frame| {
            //frame.render_widget(Paragraph::new(format!("Counter: {counter}")), frame.size());
            let area = frame.size();
            let outer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Max(area.height * 3 + 6),
                    Constraint::Min(0),
                ])
                .split(area);

            let outer_split = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(80), Constraint::Min(0)])
                .split(outer[0]);

            let game_tiles = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(outer_split[0]);

            let lower_split = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                ])
                .split(outer_split[1]);

            let player1 = Paragraph::new(
                format!("Player 1\n\n{}", app.p1_move)
            )
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                )
                //center the text vertically and horizontally
                .alignment(Alignment::Center);
            frame.render_widget(player1, game_tiles[0]);

            let player2 = Paragraph::new(
                format!("Player 2\n\n{}", app.p2_move)
            )
                .style(Style::default().fg(Color::Blue))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                )
                //center the text vertically and horizontally
                .alignment(Alignment::Center);
            frame.render_widget(player2, game_tiles[1]);

            let scoreboard = Paragraph::new(
                format!("{}:{}", app.p1_score, app.p2_score)
            )
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                )
                //center the text vertically and horizontally
                .alignment(Alignment::Center);
            frame.render_widget(scoreboard, lower_split[1]);
        })?;

        // Check for user input every 250 milliseconds
        if event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('r') => {
                            game_prog(&mut app, String::from("r"));
                        },
                        KeyCode::Char('p') => {
                            game_prog(&mut app, String::from("p"));
                        },
                        KeyCode::Char('s') => {
                            game_prog(&mut app, String::from("s"));
                        },
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // shutdown down: reset terminal back to original state
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
