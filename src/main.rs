use termion::event::Key;
use termion::event::Event;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;
use termion::cursor::DetectCursorPos;
use std::io::{Write, Read, stdin, stdout, Stdin, Stdout, StdoutLock};

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdout = stdout().into_raw_mode().unwrap();

    let mut manager = TerminalManager::new(stdout);

    for event in stdin.keys() {
        match event {
            Ok(key) => {
                match key {
                    Key::Esc => return,
                    Key::Char(i) => {
                        manager.write(i).unwrap();
                    }
                    Key::Backspace => {
                        manager.backspace().unwrap();
                    }
                    Key::Ctrl(v) => {
                        match v {
                            'a' => {
                                manager.move_caret_first().unwrap();
                            }
                            a => {}
                        }
                    }
                    _ => continue,
                }
            }
            Err(err) => {}
        }
    }
}

struct TerminalManager {
    //    stdin: Stdin,
    stdout: RawTerminal<Stdout>,
    buffer: Vec<String>,
    current_line_string: String,
}

impl TerminalManager {
    fn new(stdout: RawTerminal<Stdout>) -> TerminalManager {
        TerminalManager {
//            stdin: stdin(),
            stdout,
            buffer: Vec::new(),
            current_line_string: String::new(),
        }
    }

    fn write(&mut self, c: char) -> Result<(), std::io::Error> {
        let mut stdout = self.stdout.lock();
        self.current_line_string.push(c);
        if c == '\n' {
            write!(stdout, "{}", '\n')?;
            self.buffer.push(self.current_line_string.to_string());
            self.current_line_string.clear();
        }
        write!(stdout, "{}{}{}", termion::clear::CurrentLine, '\r', self.current_line_string)?;
        stdout.flush()?;
        Ok(())
    }

    fn backspace(&mut self) -> Result<(), std::io::Error> {
        let mut stdout = self.stdout.lock();

        let c = self.current_line_string.pop();
        if c == None {
            if let Some(v) = self.buffer.pop() {
                self.current_line_string = v;
                write!(stdout, "{}", termion::cursor::Up(1))?;
                self.current_line_string.pop();
            } else {
                return Ok(());
            }
        }
        write!(stdout, "{}{}{}", termion::clear::CurrentLine, '\r', self.current_line_string)?;
        stdout.flush()?;
        Ok(())
    }

    fn move_caret_first(&mut self) -> Result<(), std::io::Error> {
        write!(self.stdout.lock(), "{}", '\r')?;
        let (_, col) = self.stdout.cursor_pos()?;
        write!(self.stdout.lock(), "{}", termion::cursor::Goto(0, col))?;
        Ok(())
    }
}
