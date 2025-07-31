mod query;
enum State {
    Main,
    InList(String, u32)
}


use query::*;

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let mut buf = String::new();
    println!("Hello, world!");
    let conn = setup()?;
    println!("Welcome to your friendly neighbourhood to do list, start procrastinating now!");
    let mut state = State::Main;
    'outer: loop {
        buf = String::new();
        match state {
            State::Main => {
                let lists = return_lists(&conn)?;
                if lists.len() == 0 {
                    println!("You do not have any lists, create one now!");
                    println!("1.Create new list\t2.Exit");
                    std::io::stdin().read_line(&mut buf)?;
                    match buf.trim().parse::<u8>() {
                        Ok(opt) => {
                            match opt {
                                1 => {
                                    new_list(&conn)?;
                                }
                                _ => {
                                    println!("thanks for your time");
                                    std::process::exit(0);
                                }
                            }

                        }
                        Err(_) => {
                            println!("no funny business");
                            continue 'outer;
                        }
                    }
                }
                else {
                    let nlists = lists.len() as u32;
                    println!("1.Create new list\t2.View list\t3.Exit");
                    std::io::stdin().read_line(&mut buf)?;
                    match buf.trim().parse::<u8>() {
                        Ok(opt) => {
                            match opt {
                                1 => {
                                    new_list(&conn)?;
                                    continue 'outer;
                                }
                                2 => {
                                    println!("Here are your lists");
                                }
                                _ => {
                                    println!("thanks for your time");
                                    std::process::exit(0);
                                }
                            }

                        }
                        Err(_) => {
                            println!("no funny business");
                            continue 'outer;
                        }
                    }
                    
                    for list in &lists {
                        println!("{:02} : {} ", list.id, list.name);
                    }
                    println!("Pick any list by its number, click 0 to exit");
                    buf = String::new();
                    std::io::stdin().read_line(&mut buf)?;
                    match buf.trim().parse::<u32>() {
                        Ok(opt) => {
                            match opt {
                                0 => {
                                    println!("thanks for your time");
                                    std::process::exit(0);
                                }
                                x if x >=1 && x <=nlists => {
                                    let lname: Vec<&List> = lists.iter().filter(|l| l.id == x).take(1).collect();
                                    let lname: &str = &lname.get(0).unwrap().name[..];
                                    state = State::InList(String::from(lname), x);
                                    continue 'outer;
                                }
                                _ => {
                                    println!("doesn't exist pal");
                                }
                            }


                        }
                        Err(_) => {
                            println!("no funny business");
                            continue 'outer;
                        }
                    }
                }


            }
            State::InList(ref list, n) => {
                println!("{}", list);
                let tasks = return_tasks(&conn, n)?;
                if tasks.len() == 0 {
                    println!("This list has no tasks, add one now!");
                    println!("1.Create new task\t2.Exit");
                    std::io::stdin().read_line(&mut buf)?;
                    match buf.trim().parse::<u8>() {
                        Ok(opt) => {
                            match opt {
                                1 => {
                                    new_task(&conn, n)?;
                                }
                                _ => {
                                    println!("thanks for your time");
                                    std::process::exit(0);
                                }
                            }

                        }
                        Err(_) => {
                            println!("no funny business");
                            continue 'outer;
                        }
                    }


                }
                else {
                    println!("1.Add new task\t2.View tasks\t3.Update progress\t4.Delete a task\t5.Mark as completed\t6.Exit");
                    std::io::stdin().read_line(&mut buf)?;
                    match buf.trim().parse::<u32>() {
                        Ok(opt) => {
                            match opt {
                                1 => {
                                    new_task(&conn, n)?;
                                    continue 'outer;
                                }
                                2 => {
                                    disp_tasks(&tasks)?;
                                    println!("1.Go back\t2.Exit");
                                    buf = String::new();
                                    std::io::stdin().read_line(&mut buf)?;
                                    match buf.trim().parse::<u32>() {
                                        Ok(opt) => {
                                            match opt {
                                                1 => {
                                                    state = State::Main;
                                                    continue 'outer;
                                                }
                                                _ => {
                                                    println!("Thank you for your time");
                                                    std::process::exit(0);
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            println!("no funny business");
                                            continue 'outer;
                                        }
                                    }
                                

                                }
                                3 => {
                                    disp_tasks(&tasks)?;
                                    println!("Pick any task by its number, click 0 to exit");
                                    buf = String::new();
                                    std::io::stdin().read_line(&mut buf)?;
                                    match buf.trim().parse::<u32>() {
                                        Ok(opt) => {
                                            if opt == 0 {
                                                println!("Thank you for your time");
                                                std::process::exit(0);
                                            }
                                            let mut flag = false;
                                            for i in &tasks {
                                                if opt == i.id {
                                                    if i.status == "COMPLETE" {
                                                        println!("You've already completed that");
                                                        continue 'outer;
                                                    }
                                                    flag = true;
                                                    update_task(&conn, i)?;
                                                    break;
                                                }
                                            }
                                            if !flag {
                                                println!("doesn't exist mate");
                                            }
                                        }
                                        Err(_) => {
                                            println!("no funny business");
                                            continue 'outer;
                                        }
                                    }

                                }
                                4 => {
                                    disp_tasks(&tasks)?;
                                    println!("Pick any task by its number, click 0 to exit");
                                    buf = String::new();
                                    std::io::stdin().read_line(&mut buf)?;
                                    match buf.trim().parse::<u32>() {
                                        Ok(opt) => {
                                            if opt == 0 {
                                                println!("Thank you for your time");
                                                std::process::exit(0);
                                            }
                                            let mut flag = false;
                                            for i in &tasks {
                                                if opt == i.id {
                                                    flag = true;
                                                    delete_task(&conn, i)?;
                                                    break;
                                                }
                                            }
                                            if !flag {
                                                println!("doesn't exist mate");
                                            }
                                        }
                                        Err(_) => {
                                            println!("no funny business");
                                            continue 'outer;
                                        }
                                    }

                                }
                                5 => {
                                    disp_tasks(&tasks)?;
                                    println!("Pick any task by its number, click 0 to exit");
                                    buf = String::new();
                                    std::io::stdin().read_line(&mut buf)?;
                                    match buf.trim().parse::<u32>() {
                                        Ok(opt) => {
                                            if opt == 0 {
                                                println!("Thank you for your time");
                                                std::process::exit(0);
                                            }
                                            let mut flag = false;
                                            for i in &tasks {
                                                if opt == i.id {
                                                    flag = true;
                                                    if i.status == "COMPLETE" {
                                                        println!("You've already completed that");
                                                        continue 'outer;
                                                    }
                                                    mark_task(&conn, i);
                                                    break;
                                                }
                                            }
                                            if !flag {
                                                println!("doesn't exist mate");
                                            }
                                        }
                                        Err(_) => {
                                            println!("no funny business");
                                            continue 'outer;
                                        }
                                    }
                                }
                                _ => {
                                    println!("Thank you for your time");
                                    std::process::exit(0);
                                }
                            }
                        }
                        Err(_) => {
                            println!("no funny business");
                            continue 'outer;
                        }

                    }

                    
                }

            }
        }
    }

    Ok(())
}
