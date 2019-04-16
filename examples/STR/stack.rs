// Treiber stack.

use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

const ALLOCATOR_SIZE: usize = 10;

pub struct Allocator<T: Sized> {
    objects: [T; ALLOCATOR_SIZE],
    idx: usize,
}

impl<T: Sized> Allocator<T> {
    pub fn new() -> Self {
        Self {
            objects: unsafe { mem::uninitialized() },
            idx: 0,
        }
    }

    pub fn alloc(&mut self) -> *mut T {
        if self.idx < ALLOCATOR_SIZE {
            let result = &mut self.objects[self.idx] as *mut T;
            self.idx += 1;
            result
        } else {
            ptr::null_mut()
        }
    }
}

pub struct Node {
    data: AtomicUsize,
    next: AtomicPtr<Node>,
}

pub struct Stack {
    head: AtomicPtr<Node>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    // -2 if lost, -3 if oom
    #[inline(always)]
    pub fn try_push(&self, alloc: &mut Allocator<Node>, data: usize) -> isize {
        let node = alloc.alloc();

        if node.is_null() {
            return -3; // oom
        }

        let node = unsafe { &mut *node };
        node.data.store(data, Ordering::Relaxed);
        let head = self.head.load(Ordering::Relaxed);
        node.next.store(head as *mut Node, Ordering::Relaxed);
        let result = self.head.compare_exchange(head, node, Ordering::Release, Ordering::Relaxed);
        if result.is_ok() { 0 } else { -2 } // CAVEAT: memory leak if unsuccessful
    }

    // -1 if empty, -2 if lost
    #[inline(always)]
    pub fn try_pop(&self, data: &mut usize) -> isize {
        let head = self.head.load(Ordering::Acquire);

        if head.is_null() {
            return -1; // empty
        }

        let head = unsafe { &mut *head };
        *data = head.data.load(Ordering::Relaxed);
        let next = head.next.load(Ordering::Relaxed);
        let result = self.head.compare_exchange(head, next, Ordering::Release, Ordering::Relaxed);
        if result.is_ok() { 0 } else { -2 } // CAVEAT: memory leak if successful
        
    }
}

#[inline(never)]
pub fn thread0(stack_ptr: usize, alloc: &mut Allocator<Node>, X1: usize, X2: usize, X3: usize,
        result1: &mut usize, result2: &mut usize, result3: &mut usize) {
    let stack = unsafe { &*(stack_ptr as *const Stack) };
    let mut count = 1;
    let mut data = 0;

    let mut res = 0;
    for i in 0..X1 {
      if stack.try_push(alloc, count) == 0 {
        res += count;
        count *= 2;
      }
    }
    *result1 = res;

    let mut res = 0;
    for i in 0..X2 {
      if stack.try_pop(&mut data) == 0 {
        res += data;
      }
    }
    *result2 = res;

    let mut res = 0;
    for i in 0..X3 {
      if stack.try_push(alloc, count) == 0 {
        res += count;
        count *= 2;
      }
    }
    *result3 = res;
}

#[inline(never)]
pub fn thread1(stack_ptr: usize, alloc: &mut Allocator<Node>, X1: usize, X2: usize, X3: usize,
        result1: &mut usize, result2: &mut usize, result3: &mut usize) {
    let stack = unsafe { &*(stack_ptr as *const Stack) };
    let mut count = 1;
    let mut data = 0;

    let mut res = 0;
    for i in 0..X1 {
      if stack.try_push(alloc, count) == 0 {
        res += count;
        count *= 2;
      }
    }
    *result1 = res;

    let mut res = 0;
    for i in 0..X2 {
      if stack.try_pop(&mut data) == 0 {
        res += data;
      }
    }
    *result2 = res;

    let mut res = 0;
    for i in 0..X3 {
      if stack.try_push(alloc, count) == 0 {
        res += count;
        count *= 2;
      }
    }
    *result3 = res;
}

#[inline(never)]
pub fn thread2(stack_ptr: usize, alloc: &mut Allocator<Node>, X1: usize, X2: usize, X3: usize,
        result1: &mut usize, result2: &mut usize, result3: &mut usize) {
    let stack = unsafe { &*(stack_ptr as *const Stack) };
    let mut count = 1;
    let mut data = 0;

    let mut res = 0;
    for i in 0..X1 {
      if stack.try_push(alloc, count) == 0 {
        res += count;
        count *= 2;
      }
    }
    *result1 = res;

    let mut res = 0;
    for i in 0..X2 {
      if stack.try_pop(&mut data) == 0 {
        res += data;
      }
    }
    *result2 = res;

    let mut res = 0;
    for i in 0..X3 {
      if stack.try_push(alloc, count) == 0 {
        res += count;
        count *= 2;
      }
    }
    *result3 = res;
}
