use std::time::Duration;

use color_eyre::eyre::OptionExt;
use futures::{FutureExt, StreamExt};
use ratatui::crossterm::event::Event as CrosstermEvent;
use ratatui::crossterm::event::EventStream;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Crossterm(CrosstermEvent),
    App(AppEvent),
    WorktreeDeleted(bool),
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    Quit,
    TreelistUp,
    TreelistDown,
    EnterCreating,
    ExitCreating,
    SelectBranchname,
    BranchListUp,
    BranchListDown,
    TypeBranchName(CrosstermEvent),
    SelectLocation,
    CreateTree,
    EnterDeleting,
    ForceDeleteTree,
    CancelDeleting,
    DeleteTree,
}

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new(op_rx: mpsc::UnboundedReceiver<oneshot::Receiver<bool>>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = EventTask::new(sender.clone(), op_rx);
        tokio::spawn(async { actor.run().await });
        Self { sender, receiver }
    }

    pub async fn next(&mut self) -> color_eyre::Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_eyre("Failed to receive event")
    }

    pub fn send(&mut self, app_event: AppEvent) {
        let _ = self.sender.send(Event::App(app_event));
    }
}

struct EventTask {
    sender: mpsc::UnboundedSender<Event>,
    op_rx: mpsc::UnboundedReceiver<oneshot::Receiver<bool>>,
    pending_op_receiver: Option<oneshot::Receiver<bool>>,
}

const TICK_FPS: f64 = 5.0;

impl EventTask {
    fn new(
        sender: mpsc::UnboundedSender<Event>,
        op_rx: mpsc::UnboundedReceiver<oneshot::Receiver<bool>>,
    ) -> Self {
        Self {
            sender,
            op_rx,
            pending_op_receiver: None,
        }
    }

    async fn run(mut self) -> color_eyre::Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut reader = EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);

        loop {
            let tick_delay = tick.tick();
            let crossterm_event = reader.next().fuse();

            let pending = async {
                match self.pending_op_receiver.as_mut() {
                    Some(rx) => rx.await,
                    None => std::future::pending().await,
                }
            };

            tokio::select! {
                _ = self.sender.closed() => {
                    break;
                }
                _ = tick_delay => {
                    self.send(Event::Tick);
                }
                Some(Ok(evt)) = crossterm_event => {
                    self.send(Event::Crossterm(evt));
                }
                success = pending => {
                    tracing::info!("in pending branch");
                    self.pending_op_receiver = None;
                    self.send(Event::WorktreeDeleted(success.unwrap_or(false)));
                }
                Some(rx) = self.op_rx.recv() => {
                    self.pending_op_receiver = Some(rx);
                }
            }
        }
        Ok(())
    }

    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
