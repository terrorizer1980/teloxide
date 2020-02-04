// https://rust-lang.github.io/async-book/02_execution/04_executor.html

use {
    futures::{
        future::{FutureExt, BoxFuture},
        task::{ArcWake, waker_ref},
    },
    std::{
        future::Future,
        sync::{Arc, Mutex},
        sync::mpsc::{sync_channel, SyncSender, Receiver},
        task::{Context, Poll},
        time::Duration,
    },
    // The timer we wrote in the previous section:
    timer_future::TimerFuture,
};
use std::collections::HashMap;

use crate::dispatching::Dispatcher;
use crate::bot::Bot;
use reqwest::Client;

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // Create a `LocalWaker` from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>` is a type alias for
                // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                // We can get a `Pin<&mut dyn Future + Send + 'static>`
                // from it by calling the `Pin::as_mut` method.
                if let Poll::Pending = future.as_mut().poll(context) {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

pub struct Session<Stor> {
    bots: HashMap<Token, Bot>,
    client: Client,
    executor: Executor,
    storage: Stor,
}

impl Session {
    pub fn asociate_bot(&mut self, bot: Bot) -> Option<&Bot> {
        self.bots.get(bot.token())
    }
    pub fn idle(&mut self) {
        self.executor.run()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self {
            bots: HashMap::new(),
            client: Client::new(),
            executor: ,

        }
    }
}

/// Create single bot session 
pub fn new_single_bot_session<S>(
    tok_convertable, 
    Disp: Dispatcher
) -> Session 
{
    let mut ses = Session::default();
    ses.asociate_bot(Bot::new(tok_convertable));
    ses
}

/// Create bot pool session with shared storage and common dispathcer
pub fn new_bot_pool_session<TokCon: Iterator, Disp: Dispatcher>(
    token_container: TokCon, 
    dispathcer: Disp
) -> Session 
{    
    let mut ses = Session::default()
    for tok in token_container {
        ses.asociate_bot(Bot::new(tok))
    }
    ses
}
