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
                    println!("1.Create new list\t2.Add task\t3.Exit");
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
                    
                }

            }
        }
    }


    

    Ok(())
}
