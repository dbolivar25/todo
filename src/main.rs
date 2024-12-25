use clap::Parser;
use reedline::{Reedline, Signal};
use todo::{
    cli::{Args, Command},
    error::Result,
    prompt::CustomPrompt,
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
            repo.add(name, description, weight, days_to_start, days_to_complete)?;
            println!("Todo item added successfully!");
        }
        Command::Remove { name } => {
            repo.remove(&name)?;
            println!("Todo item removed successfully!");
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
                name,
                new_name,
                description,
                weight,
                days_to_start,
                days_to_complete,
            )?;
            println!("Todo item updated successfully!");
        }
        Command::Complete { name } => {
            repo.complete(&name)?;
            println!("Todo item marked as complete!");
        }
        Command::List {
            weight,
            completed,
            sort_by_deadline,
            sort_by_weight,
        } => {
            let items = repo.list(weight, completed, sort_by_deadline, sort_by_weight)?;

            if items.is_empty() {
                println!("No todo items found.");
                return Ok(());
            }

            println!("\nTodo Items:");
            for item in items {
                println!("{}", item);
                println!("----------------------------------------");
            }
        }
    }
    Ok(())
}

fn run_repl(repo: &mut Repo) -> Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = CustomPrompt;

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                // Parse the input line as if it were command line arguments
                match shlex::split(&buffer) {
                    Some(arg_strings) => match Args::try_parse_from(arg_strings) {
                        Ok(Args {
                            command: Some(command),
                        }) => {
                            if let Err(e) = execute_command(repo, command) {
                                eprintln!("Error: {}", e);
                            }
                        }
                        Ok(Args { command: None }) => {}
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    },
                    None => {
                        eprintln!("Error: Invalid command syntax");
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
