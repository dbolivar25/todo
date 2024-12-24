use clap::Parser;
use todo::{
    cli::{Args, Command},
    error::Result,
    repo::Repo,
};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut repo = Repo::new()?;

    match args.command {
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
