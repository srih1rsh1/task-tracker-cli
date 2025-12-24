mod task_tracker;

use std::{env,io};

use crate::task_tracker::{Status, TaskTracker, Update};

fn main() -> io::Result<()> {
    //Application StartUp
    let _ = TaskTracker::builder()?;
    let args: Vec<String> = env::args().collect();

    let application_info = "
-------------------          
Welcome To Task-CLI
-------------------

Try Passing a command. Here are the available commands:

* add               - Use this command to add a task to the tracker.

* update            - Use this command to update the task description.

* delete            - Use this command to delete a task from the task tracker.

* list              - Use this command to list all the tasks or list by status of the task.

* mark-in-progress  - Use this update the status of the task to InProgress.

* mark-done         - Use this to update the status of the task to Done.

* mark-todo         - Use this to update the status of the task to ToDo ";

    let unknown_command =
        "The provided command is not in correct syntax or try provided valid arguments.

Use --help or -h to find the allowed commands.";

    match args.len() {
        1 => {
            // let app = TaskTracker::builder()?;
            println!("{}", application_info);
            Ok(())
        }
        2 => match args[1].parse::<String>() {
            Ok(s) => {
                if s == "help" || s == "h" || s == "--help" || s == "-h" {
                    println!("{}", application_info);
                    Ok(())
                } else if s == "list" || s == "ls" {
                    TaskTracker::list_all();
                    Ok(())
                } else {
                    eprintln!("Error: {}", unknown_command);
                    Ok(())
                }
            }
            Err(_) => Ok(()),
        },
        3 => {
            let sub_command = &args[1];
            let argument = &args[2];
            match &sub_command[..] {
                "add" => {
                    let description: String = match argument.parse() {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("error: second argument not an string");
                            return Ok(());
                        }
                    };

                    let id = TaskTracker::add(description)?;

                    println!("Task added successfully (ID: {})", id);

                    Ok(())
                }

                "list" => {
                    let _ = TaskTracker::builder();
                    let argument: String = match argument.parse() {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("error: second argument not an string");
                            return Ok(());
                        }
                    };

                    let status: Status = {
                        if argument == "ToDo" {
                            Status::ToDo
                        } else if argument == "Done" {
                            Status::Done
                        } else if argument == "InProgress" {
                            Status::InProgress
                        } else if argument == "--help" || argument == "-h" {
                            println!(
                                "The Allowed Status type to list are:

* ToDo
* InProgress
* Done"
                            );
                            return Ok(());
                        } else {
                            eprintln!("Invalid Status Type: {}", argument);
                            return Ok(());
                        }
                    };
                    TaskTracker::list_by_status(status);
                    Ok(())
                }
                "mark-in-progress" => {
                    let id: u128 = match argument.parse() {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("error: second argument not an string");
                            return Ok(());
                        }
                    };
                    TaskTracker::update(id, Update::Status(Status::InProgress));
                    Ok(())
                }
                "mark-Done" => {
                    let id: u128 = match argument.parse() {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("error: second argument not an string");
                            return Ok(());
                        }
                    };
                    TaskTracker::update(id, Update::Status(Status::Done));
                    Ok(())
                }
                "mark-done" => {
                    let id: u128 = match argument.parse() {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("error: second argument not an string");
                            return Ok(());
                        }
                    };
                    TaskTracker::update(id, Update::Status(Status::Done));
                    Ok(())
                }
                "mark-todo" => {
                    let id: u128 = match argument.parse() {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("error: second argument not an string");
                            return Ok(());
                        }
                    };
                    TaskTracker::update(id, Update::Status(Status::ToDo));
                    Ok(())
                }
                "delete" => {
                    let id: u128 = match argument.parse() {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("error: second argument not an string");
                            return Ok(());
                        }
                    };
                    TaskTracker::delete_task(id);
                    Ok(())
                }

                _ => {
                    eprintln!("Error: {}", unknown_command);
                    Ok(())
                },
            }
        }
        4 => {
            let sub_command = &args[1];
            let id = &args[2];
            let argument = &args[3];
            match &sub_command[..] {
                "update" => {
                    let id: u128 = match id.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("error: second argument not an integer");
                            return Ok(());
                        }
                    };
                    let description: String = match argument.parse() {
                        Ok(s) => s,
                        Err(_) => {
                            eprint!("error: second argument not an string");
                            return Ok(());
                        }
                    };
                    TaskTracker::update(id, task_tracker::Update::Description(description));

                    Ok(())
                }
                _ => {
                    eprintln!("Error: {}", unknown_command);
                    Ok(())
                },
            }
        }
        _ => {
                    eprintln!("Error: {}", unknown_command);
                    Ok(())
                },
    }
}
