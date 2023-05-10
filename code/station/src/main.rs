use std::io::stdout;

use crossterm::{event::{read, Event, KeyCode, KeyEventKind, KeyEvent}, style::{self, Stylize}, execute};
use scanner::{ScanBuilder, ScanCount};
use tokio::{io::AsyncWriteExt, time::interval, sync::broadcast};
mod scanner;

struct Command(u8);
impl From<Command> for u8{
    fn from(value: Command) -> Self {
        value.0
    }
}
impl Command{
    const TILT_UP: Self = Self(0);
    const TILT_DOWN: Self = Self(1);
    const TILT_OFF: Self = Self(2);
    const PAN_RIGHT: Self = Self(3);
    const PAN_LEFT: Self = Self(4);
    const PAN_OFF: Self = Self(5);
}

fn main() {
    pretty_env_logger::init();
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("Could not build tokio runtime");

    let (evt_tx, evt_rx) = broadcast::channel::<KeyEvent>(1000);
    runtime.spawn(comms(evt_rx));

    loop{
        if let Ok(Event::Key(event)) = read(){
            // println!("Key event {:?}", event);
            let _ = evt_tx.send(event);
        }
    }
}

async fn comms(mut evt_rx: broadcast::Receiver<KeyEvent>){

    // let mut up_state = false;
    // let mut down_state = false;
    // let mut left_state = false;
    // let mut right_state = false;
    let mut stdout = stdout();

    let _ = execute!(stdout, style::PrintStyledContent("Scanning for nano".blue()));
    let scan = ScanBuilder::default().scan_count(ScanCount::Infinite).add_port(1000).dispatch();

    let mut tgt = match scan.recv_async().await{
        Ok(s) => s,
        Err(_) => return
    };

    drop(scan);

    let _ = execute!(stdout, style::PrintStyledContent("Connected!".magenta()));
    
    loop{

        match evt_rx.recv().await{
            Ok(KeyEvent { code: KeyCode::Up, modifiers: _, kind: KeyEventKind::Press, state: _ }) => {
                let _ = tgt.write_u8(Command::TILT_UP.into()).await;
                let _ = execute!(stdout, style::PrintStyledContent("Sent command".magenta()));
            }
            Ok(KeyEvent { code: KeyCode::Up, modifiers: _, kind: KeyEventKind::Release, state: _ }) => {
                let _ = tgt.write_u8(Command::TILT_OFF.into()).await;
                let _ = execute!(stdout, style::PrintStyledContent("Sent command".magenta()));
            }
            Ok(KeyEvent { code: KeyCode::Down, modifiers: _, kind: KeyEventKind::Press, state: _ }) => {
                let _ = tgt.write_u8(Command::TILT_DOWN.into()).await;
                let _ = execute!(stdout, style::PrintStyledContent("Sent command".magenta()));
            }
            Ok(KeyEvent { code: KeyCode::Down, modifiers: _, kind: KeyEventKind::Release, state: _ }) => {
                let _ = tgt.write_u8(Command::TILT_OFF.into()).await;
                let _ = execute!(stdout, style::PrintStyledContent("Sent command".magenta()));
            }
            Ok(KeyEvent { code: KeyCode::Right, modifiers: _, kind: KeyEventKind::Press, state: _ }) => {
                let _ = tgt.write_u8(Command::PAN_RIGHT.into()).await;
                let _ = execute!(stdout, style::PrintStyledContent("Sent command".magenta()));
            }
            Ok(KeyEvent { code: KeyCode::Right, modifiers: _, kind: KeyEventKind::Release, state: _ }) => {
                let _ = tgt.write_u8(Command::PAN_OFF.into()).await;
                let _ = execute!(stdout, style::PrintStyledContent("Sent command".magenta()));
            }
            Ok(KeyEvent { code: KeyCode::Left, modifiers: _, kind: KeyEventKind::Press, state: _ }) => {
                let _ = tgt.write_u8(Command::PAN_LEFT.into()).await;
                let _ = execute!(stdout, style::PrintStyledContent("Sent command".magenta()));
            }
            Ok(KeyEvent { code: KeyCode::Left, modifiers: _, kind: KeyEventKind::Release, state: _ }) => {
                let _ = tgt.write_u8(Command::PAN_OFF.into()).await;
                let _ = execute!(stdout, style::PrintStyledContent("Sent command".magenta()));
            }
            _ => {}
        }
    }

    
}
