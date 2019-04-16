use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct Spinlock {
  locked: AtomicIsize,
}

impl Spinlock {
  #[inline(always)]
  pub fn lock(&self) -> bool {
    match self.locked.compare_exchange(0, 1, Acquire, Relaxed) {
      Ok(_) => true,
      Err(_) => false,
    }
  }

  #[inline(always)]
  pub fn unlock(&self) {
    self.locked.store(0, Release);
  }
}

pub fn thread0(l: &Spinlock, x: i64, data: &AtomicIsize) {
  for _i in 0..x {
    if l.lock() {
      let d = data.load(Relaxed);
      data.store(d + 42, Relaxed);
      l.unlock();
      break;
    }
  }
}

pub fn thread1(l: &Spinlock, x: i64, data: &AtomicIsize) {
  for _i in 0..x {
    if l.lock() {
      let d = data.load(Relaxed);
      data.store(d + 42, Relaxed);
      l.unlock();
      break;
    }
  }
}

pub fn thread2(l: &Spinlock, x: i64, data: &AtomicIsize) {
  for _i in 0..x {
    if l.lock() {
      let d = data.load(Relaxed);
      data.store(d + 42, Relaxed);
      l.unlock();
      break;
    }
  }
}
