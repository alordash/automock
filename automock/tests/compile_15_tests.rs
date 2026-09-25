#![allow(unused)]
use automock::*;
use std::pin::Pin;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[mock]
struct MyFuture {
    atomic_result: Arc<AtomicI32>,
    atomic_waker: Arc<Mutex<Option<Waker>>>,
    thread: JoinHandle<()>,
}

#[mock(base)]
impl MyFuture {
    pub fn new(value: i32) -> MyFuture {
        let atomic_result = Arc::new(AtomicI32::new(0));
        let atomic_waker = Arc::new(Mutex::new(None));
        let atomic_result_clone = atomic_result.clone();
        let atomic_waker_clone = atomic_waker.clone();
        MyFuture {
            atomic_waker,
            atomic_result,
            thread: thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));

                atomic_result_clone.store(value, Ordering::SeqCst);

                let mut waker_lock = atomic_waker_clone.lock().unwrap();
                if let Some(waker) = waker_lock.take() {
                    waker.wake();
                }
            }),
        }
    }
}

#[mock(base)]
async fn work() -> i32 {
    tokio::time::sleep(Duration::from_secs(1));
    return 12;
}

#[mock]
#[allow(unused)]
async fn work_return() -> i32 {
    tokio::time::sleep(Duration::from_secs(1));
    return 12;
}

#[mock]
#[allow(unused)]
async fn work_nothing() {}

#[mock(base)]
#[allow(unused)]
async fn work_base_nothing() {}

#[mock(base)]
impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        if self.thread.is_finished() {
            Poll::Ready(self.atomic_result.load(Ordering::SeqCst))
        } else {
            *self.atomic_waker.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[test]
fn compile() {}
