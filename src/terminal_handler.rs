use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;

use crossterm::event::{self, Event as CrossEvent, KeyEvent};

#[derive(Debug, Clone, Copy)]
pub enum Events {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventsHandler {
    sender: mpsc::Sender<Events>,
    receiver: mpsc::Receiver<Events>,
    handler: thread::JoinHandle<()>,
}

impl EventsHandler {
    pub fn new(tick: u64) -> Self {
        let tick_duration = Duration::from_millis(tick);
        let (sender, receiver) = mpsc::channel();

        let handler = thread::spawn({
            let sender = sender.clone();

            move || {
                let mut prev_tick = Instant::now();

                loop {
                    let elapsed = prev_tick.elapsed();
                    let timeout = tick_duration.saturating_sub(elapsed);

                    if let Ok(true) = event::poll(timeout) {
                        if let Ok(event) = event::read() {
                            let _ = match event {
                                CrossEvent::Key(e) if e.kind == event::KeyEventKind::Press => {
                                    sender.send(Events::Key(e))
                                }
                                CrossEvent::Resize(w, h) => sender.send(Events::Resize(w, h)),
                                _ => Ok(()),
                            };
                        }
                    }

                    if elapsed >= tick_duration {
                        let _ = sender.send(Events::Tick);
                        prev_tick = Instant::now();
                    }
                }
            }
        });

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> Result<Events> {
        Ok(self.receiver.recv()?)
    }
}
