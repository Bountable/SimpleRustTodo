//The tasks module will represent our tasks, and how we will save and access them.
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;



#[derive(Debug, Deserialize, Serialize)]
pub struct Task{
    pub text : String, //text stores the task description, like "pay the bills".
    
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

//define implementations on type Task
impl Task{
    pub fn new(text: String) -> Task{
        let created_at: DateTime<Utc> = Utc::now();
        Task{ text, created_at}
    }

}

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Result;
use std::path::PathBuf;



//add_task also requires a Task argument. That argument specifies the task that will be added to the list.
pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let  file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(journal_path)?;
    let mut tasks = collect_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}


use std::io::{Error, ErrorKind, Seek, SeekFrom};  // Include the `Error` type.
//complete_task requires a task_position argument to indicate which Task will be removed. When a task is removed, that means it's completed.
pub fn complete_task(journal_path: PathBuf,  task_position: usize) -> Result<()> {
    //open file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    
    // Consume file's contents as a vector of tasks.
    let mut tasks = collect_tasks(&file)?;


    //try Remove Task
    if task_position ==0 || task_position > tasks.len(){
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position -1);

    //rewind and truncate file
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    //write modified task list back into file
    serde_json::to_writer(file, &tasks)?;
    Ok(())


} 

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(tasks)
}

//list_tasks doesn't need any additional information. It will just present to the user all tasks currently stored in the journal file, in a pretty format.
pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // Parse the file and collect the tasks.
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at) // a left-aligned string padded with 50 spaces.  [{}]: the date and time the task was created, inside brackets.
    }
}


