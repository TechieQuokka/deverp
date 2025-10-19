// Resource CLI commands

use super::commands::ResourceCommand;
use crate::Result;

/// Handle resource commands
pub async fn handle(command: ResourceCommand) -> Result<()> {
    match command {
        ResourceCommand::Create => {
            println!("Resource Create - Not yet implemented");
            println!("Usage: deverp resource create --name <NAME> --type <TYPE> [OPTIONS]");
            Ok(())
        }
        ResourceCommand::List => {
            println!("Resource List - Not yet implemented");
            println!("Usage: deverp resource list [--type <TYPE>] [--status <STATUS>]");
            Ok(())
        }
        ResourceCommand::Show => {
            println!("Resource Show - Not yet implemented");
            println!("Usage: deverp resource show <ID>");
            Ok(())
        }
        ResourceCommand::Update => {
            println!("Resource Update - Not yet implemented");
            println!("Usage: deverp resource update <ID> [OPTIONS]");
            Ok(())
        }
        ResourceCommand::Delete => {
            println!("Resource Delete - Not yet implemented");
            println!("Usage: deverp resource delete <ID>");
            Ok(())
        }
        ResourceCommand::Link => {
            println!("Resource Link - Not yet implemented");
            println!("Usage: deverp resource link --project-id <ID> --resource-id <ID>");
            Ok(())
        }
        ResourceCommand::Unlink => {
            println!("Resource Unlink - Not yet implemented");
            println!("Usage: deverp resource unlink --project-id <ID> --resource-id <ID>");
            Ok(())
        }
        ResourceCommand::Usage => {
            println!("Resource Usage - Not yet implemented");
            println!("Usage: deverp resource usage [<RESOURCE_ID>]");
            Ok(())
        }
    }
}
