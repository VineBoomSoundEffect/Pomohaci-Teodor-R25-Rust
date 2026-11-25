use std::{io, fs};

trait Command {
    fn get_name(&self);
    fn exec(&self, state: &mut TerminalState);
}

struct PingCommand {
}
impl Command for PingCommand {
    fn get_name(&self) {
        println!("ping");
    }
    fn exec(&self, state: &mut TerminalState) {
        println!("pong");
    }
}

struct CountCommand {
    nargs: usize,
}
impl Command for CountCommand {
    fn get_name(&self) {
        println!("count");
    }
    fn exec(&self, state: &mut TerminalState) {
        println!("nargs: {}", self.nargs);
    }
}

struct TimesCommand {
    nargs: usize,
}
impl Command for TimesCommand {
    fn get_name(&self) {
        println!("times");
    }
    fn exec(&self, state: &mut TerminalState) {
        state.times += 1;
        println!("times_count now: {}", state.times);
    }
}

struct TerminalState {
    times: usize
}
struct Terminal {
    commands: Vec<Box<dyn Command>>,
    // metoda exec() din Command trebuie sa modifice starea din terminal.
    // As fi putut lasa doar times, iar exec sa aiba signatura (&self, usize),
    // dar am lasat asa pentru generalitate
    state: TerminalState,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal{commands: Vec::<Box<dyn Command>>::new(), state: TerminalState{times: 0}}
    }
    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
    fn run(&mut self) {
        for command in &mut self.commands {
            command.exec(&mut self.state);
        }
    }
}

fn get_words_from_line(line: String) -> Vec<String> {
    let mut words = Vec::<String>::new();
    for word in line.split(" ") {
        if !word.is_empty() {
            words.push(word.to_string());
        }
    }
    words
}

fn read_commands_from_file(terminal: &mut Terminal, file: String) {
    let contents = fs::read_to_string(file);
    match contents {
        Ok(contents) => {
            for line in contents.lines() {
                let words = get_words_from_line(line.to_string());
                if words.len() != 0 {
                    match words[0].as_str() {
                        "ping" => {
                            terminal.register(Box::new(PingCommand {}));
                        }
                        "count" => {
                            terminal.register(Box::new(CountCommand {nargs: words.len()-1}));
                        }
                        "times" => {
                            terminal.register(Box::new(TimesCommand {nargs: words.len()-1}));
                        }
                        "stop" => {break;}
                        _ => {}
                    }
                }
            }
        }
        Err(_) => { return; }
    }
}

fn main() {
    //let mut v = Vec::<i32>::new();
    //v.push(1);
    //v.push(3);
    //v.push(6);
    //println!("{}", v.last().unwrap());
    let mut terminal = Terminal::new();
    read_commands_from_file(&mut terminal, "input.txt".to_string());
    terminal.run();
}
