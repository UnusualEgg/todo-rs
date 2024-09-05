use clap::ArgAction;
use clap_derive::{Args, Parser, Subcommand};

use crate::task;

#[derive(Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub commands: Commands,
}

//relating to tasks
#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(alias = "a")]
    Add(Add),
    #[clap(alias = "ls")]
    List,
    #[clap(alias = "rm")]
    Remove(IDArg),
    #[clap(alias = "f")]
    #[command(subcommand)]
    Folder(FolderCommands),
    #[clap(alias = "c")]
    Complete(IDArg),
    #[clap(alias = "r")]
    Rename(RenameArgs),
}

//relating to folders unless specified
#[derive(Debug, Subcommand)]
pub enum FolderCommands {
    #[clap(alias = "a")]
    Add(FolderAddArgs),
    #[clap(alias = "rm")]
    Remove(IDArg),
    #[clap(alias = "ls")]
    List(List),
    #[clap(alias = "desc", alias = "d")]
    SetDesc(FolderSetDescArgs),
    #[clap(alias = "ta")]
    Task(FolderAddTaskArgs),
    #[clap(alias = "trm")]
    TaskRemove(FolderAddTaskArgs),
}
#[derive(Args, Debug)]
pub struct FolderAddTaskArgs {
    pub folder: usize,
    #[command(flatten)]
    pub task: ArgTaskID,
}
#[derive(Args, Debug)]
pub struct RenameArgs {
    #[command(flatten)]
    pub id: ArgTaskID,
    pub new: String,
}
#[derive(Args, Debug)]
pub struct FolderSetDescArgs {
    #[command(flatten)]
    pub id: ArgTaskID,
    #[arg(required = false)]
    pub desc: String,
}
#[derive(Args, Debug)]
pub struct FolderAddArgs {
    pub name: String,
    #[arg(required = false, default_missing_value = "")]
    pub desc: String,
}

#[derive(Args, Debug)]
pub struct List {
    #[arg(long="nodesc",short='d',
        help="Exclude printing descriptions of folders",
        action=ArgAction::SetFalse,
        default_missing_value="true")]
    pub desc: bool,
    #[arg(long, short)]
    pub tasks: bool,
}
#[derive(Args, Debug)]
pub struct Add {
    #[arg(help = "task name")]
    pub name: String,
    #[arg(help = "include current path", long, short)]
    pub path: bool,
}
#[derive(Args, Debug, Clone)]
#[group(required = true)]
pub struct ArgTaskID {
    #[arg(short, long, conflicts_with = "all")]
    pub name: Vec<String>,
    #[arg(short, long, conflicts_with = "all")]
    pub id: Vec<task::ID>,
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct IDArg {
    #[command(flatten)]
    pub id: ArgTaskID,
}
