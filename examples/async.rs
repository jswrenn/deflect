#![allow(
    dead_code,
    clippy::disallowed_names,
)]

use deflect::Reflect;
use std::error::Error;
use std::sync::Arc;
use std::task::{RawWaker, RawWakerVTable, Waker};

macro_rules! waker_vtable {
    ($ty:ident) => {
        &RawWakerVTable::new(
            clone_arc_raw::<$ty>,
            wake_arc_raw::<$ty>,
            wake_by_ref_arc_raw::<$ty>,
            drop_arc_raw::<$ty>,
        )
    };
}

pub trait ArcWake {
    fn wake(self: Arc<Self>);

    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.clone().wake()
    }

    fn into_waker(wake: Arc<Self>) -> Waker
    where
        Self: Sized,
    {
        let ptr = Arc::into_raw(wake) as *const ();

        unsafe { Waker::from_raw(RawWaker::new(ptr, waker_vtable!(Self))) }
    }
}

unsafe fn increase_refcount<T: ArcWake>(data: *const ()) {
    // Retain Arc by creating a copy
    let arc: Arc<T> = Arc::from_raw(data as *const T);
    let arc_clone = arc.clone();
    // Forget the Arcs again, so that the refcount isn't decrased
    let _ = Arc::into_raw(arc);
    let _ = Arc::into_raw(arc_clone);
}

unsafe fn clone_arc_raw<T: ArcWake>(data: *const ()) -> RawWaker {
    increase_refcount::<T>(data);
    RawWaker::new(data, waker_vtable!(T))
}

unsafe fn drop_arc_raw<T: ArcWake>(data: *const ()) {
    // Drop Arc
    let _: Arc<T> = Arc::from_raw(data as *const T);
}

unsafe fn wake_arc_raw<T: ArcWake>(data: *const ()) {
    let arc: Arc<T> = Arc::from_raw(data as *const T);
    ArcWake::wake(arc);
}

unsafe fn wake_by_ref_arc_raw<T: ArcWake>(data: *const ()) {
    let arc: Arc<T> = Arc::from_raw(data as *const T);
    ArcWake::wake_by_ref(&arc);
    let _ = Arc::into_raw(arc);
}

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{self, AtomicUsize};
use std::task::{Context, Poll};

struct Counter {
    wakes: AtomicUsize,
}

impl ArcWake for Counter {
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.wakes.fetch_add(1, atomic::Ordering::SeqCst);
    }
}

struct WakeOnceThenComplete(bool, u8);

impl Future for WakeOnceThenComplete {
    type Output = u8;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
        if self.0 {
            Poll::Ready(self.1)
        } else {
            cx.waker().wake_by_ref();
            self.0 = true;
            Poll::Pending
        }
    }
}

fn poll_once<F: Future<Output = u8>>(fut: Pin<&mut F>) -> Poll<u8> {
    let mut fut = Box::pin(fut);
    let counter = Arc::new(Counter {
        wakes: AtomicUsize::new(0),
    });
    let waker = ArcWake::into_waker(counter);
    let mut cx = Context::from_waker(&waker);
    fut.as_mut().poll(&mut cx)
}

fn base() -> WakeOnceThenComplete {
    WakeOnceThenComplete(false, 1)
}

async fn await1_level1() -> u8 {
    base().await
}

async fn await2_level1() -> u8 {
    base().await + base().await
}

async fn await3_level1() -> u8 {
    base().await + base().await + base().await
}

async fn await3_level2() -> u8 {
    let foo = Box::pin(await3_level1());
    let bar = await3_level1();
    let baz = await3_level1();
    foo.await + bar.await + baz.await
}

async fn await3_level3() -> u8 {
    await3_level2().await + await3_level2().await + await3_level2().await
}

async fn await3_level4() -> u8 {
    await3_level3().await + await3_level3().await + await3_level3().await
}

async fn await3_level5() -> u8 {
    let foo = await3_level4();
    let bar = await3_level4();
    let baz = await3_level4();

    let x = foo.await;
    let y = x + bar.await;
    
    y + baz.await
}

fn poll<F: Future>(_f: F) -> impl Reflect {
    <F as Future>::poll
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut task = Box::pin(await3_level5());
    let mut i = 0;
    loop {
        let res = poll_once(task.as_mut());
        let erased: &dyn Reflect = &task;
        let context = deflect::default_provider()?;

        let value = erased.reflect(&context)?;

        print!("\x1B[2J\x1B[1;1H");
        println!("STEP {i}");
        println!("{value:#}");

        if res.is_ready() {
            break;
        }

        i += 1;

        //std::io::stdin().read_line(&mut String::new()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
