use rand::prelude::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use futures::executor::*;
use rand::prelude::*;
use std::future::*;
use std::pin::Pin;
use std::task::{Context, Poll};

struct DeleteFuture;
impl Future for DeleteFuture {
    type Output = Result<String, String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("poll called -------------------");

        let mut rng = rand::thread_rng();
        let i: i32 = rng.gen_range(0, 10);

        std::thread::sleep(std::time::Duration::from_millis(1));

        if i < 5 {
            Poll::Ready(Ok("success".to_string()))
        } else {
            Poll::Ready(Err("failure".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn playground() {
        let result = block_on(DeleteFuture);
        println!("result {:?}", result);
    }
}
