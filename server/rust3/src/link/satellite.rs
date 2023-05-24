use tokio::sync::mpsc;

pub struct Satellite<T> {
    pub receiver: mpsc::Receiver<T>,
    pub sender: mpsc::Sender<T>,
}

impl<T> Satellite<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        Satellite { receiver: rx, sender: tx }
    }
}