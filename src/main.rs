use clap::Parser;
use nu_ansi_term::{Color, Style};
use reedline::{DefaultHinter, Reedline, Signal};
use todo::{
    cli::{Args, Command},
    error::Result,
    prompt::TodoPrompt,
    repo::Repo,
};

fn execute_command(repo: &mut Repo, command: Command) -> Result<()> {
    match command {
        Command::Add {
            name,
            description,
            weight,
            days_to_start,
            days_to_complete,
        } => {
            repo.add(
                name.clone(),
                description.clone(),
                weight,
                days_to_start,
                days_to_complete,
            )?;
            println!("✓ Added new task: {}", name);
            if let Some(desc) = description {
                println!("  Description: {}", desc);
            }
            if let Some(w) = weight {
                println!("  Weight: {}", w);
            }
            if let Some(start) = days_to_start {
                println!("  Start in: {} days", start);
            }
            if let Some(complete) = days_to_complete {
                println!("  Complete in: {} days", complete);
            }
        }
        Command::Remove { name } => {
            repo.remove(&name)?;
            println!("✓ Removed task: {}", name);
        }
        Command::Edit {
            name,
            new_name,
            description,
            weight,
            days_to_start,
            days_to_complete,
        } => {
            repo.edit(
                name.clone(),
                new_name.clone(),
                description.clone(),
                weight,
                days_to_start,
                days_to_complete,
            )?;
            println!("✓ Updated task: {}", name);
            if let Some(new) = new_name {
                println!("  New name: {}", new);
            }
            if let Some(desc) = description {
                println!("  New description: {}", desc);
            }
            if let Some(w) = weight {
                println!("  New weight: {}", w);
            }
            if let Some(start) = days_to_start {
                println!("  New start in: {} days", start);
            }
            if let Some(complete) = days_to_complete {
                println!("  New complete in: {} days", complete);
            }
        }
        Command::Complete { name } => {
            repo.complete(&name)?;
            println!("✓ Marked as complete: {}", name);
        }
        Command::List {
            weight,
            completed,
            sort_by_deadline,
            sort_by_weight,
        } => {
            let items = repo.list(weight, completed, sort_by_deadline, sort_by_weight)?;
            if items.is_empty() {
                println!("No tasks");
                if let Some(w) = weight {
                    println!("  (filtered by weight: {})", w);
                }
                if completed {
                    println!(
                        "  (showing {} tasks)",
                        if completed { "completed" } else { "pending" }
                    );
                }
                return Ok(());
            }

            // Print list header with filter information
            println!("Tasks");
            if let Some(w) = weight {
                println!("  Weight filter: {}", w);
            }
            if completed {
                println!(
                    "  Showing: {} tasks",
                    if completed { "completed" } else { "pending" }
                );
            }
            if sort_by_deadline {
                println!("  Sorted by: deadline");
            } else if sort_by_weight {
                println!("  Sorted by: weight");
            }
            println!();

            // Print tasks
            for item in items {
                println!("{}", item);
                println!("{}", str::repeat("─", 40));
            }
        }
    }
    Ok(())
}

fn run_repl(repo: &mut Repo) -> Result<()> {
    let mut line_editor = Reedline::create().with_hinter(Box::new(
        DefaultHinter::default().with_style(Style::new().italic().fg(Color::LightGray)),
    ));
    let prompt = TodoPrompt;

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                // Parse the input line as if it were command line arguments
                match shlex::split(&buffer) {
                    Some(mut arg_strings) => {
                        arg_strings.insert(0, "todo".to_string());

                        match Args::try_parse_from(arg_strings) {
                            Ok(Args {
                                command: Some(command),
                            }) => {
                                if let Err(e) = execute_command(repo, command) {
                                    eprintln!("error: {}", e);
                                }
                            }
                            Ok(Args { command: None }) => {}
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
                    }
                    None => {
                        eprintln!("error: invalid command syntax");
                    }
                }
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                break Ok(());
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break Ok(());
            }
        }
    }
}

fn main() -> Result<()> {
    if let Args {
        command: Some(command),
    } = Args::parse()
    {
        let mut repo = Repo::new()?;
        execute_command(&mut repo, command)?;

        Ok(())
    } else {
        let mut repo = Repo::new()?;
        run_repl(&mut repo)?;

        Ok(())
    }
}
