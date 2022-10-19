#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

pub struct Sender<T> {
    // TODO: your code goes here.
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn is_closed(&self) -> bool {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    // TODO: your code goes here.
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn close(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // TODO: your code goes here.
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{channel, ReceiveError};

    use std::{error::Error, iter::repeat};

    #[derive(Debug)]
    struct Int(usize);

    #[test]
    fn simple() {
        let (sender, mut receiver) = channel::<Int>();
        for k in 0..10 {
            for i in k..k + 10 {
                sender.send(Int(i % 10)).unwrap();
            }
            for i in k..k + 10 {
                assert_eq!(receiver.recv().unwrap().0, i % 10);
            }
        }
    }

    #[test]
    fn sender_clone() {
        let (sender, mut receiver) = channel::<Int>();
        let senders = repeat(sender).take(10).collect::<Vec<_>>();
        for k in 0..10 {
            for i in k..k + 10 {
                senders[i % 10].send(Int(i % 10)).unwrap();
            }
            for i in k..k + 10 {
                assert_eq!(receiver.recv().unwrap().0, i % 10);
            }
        }
    }

    #[test]
    fn close() {
        let (sender, mut receiver) = channel::<Int>();
        let senders = repeat(sender).take(10).collect::<Vec<_>>();
        for i in 0..10 {
            senders[0].send(Int(i)).unwrap();
        }
        receiver.close();

        for i in 0..10 {
            assert!(senders[i].is_closed());
            let err = senders[i].send(Int(i)).unwrap_err();
            assert!(Error::source(&err).is_none());
            assert_eq!(err.value.0, i);
        }

        for i in 0..10 {
            assert_eq!(receiver.recv().unwrap().0, i);
        }

        let err = receiver.recv().unwrap_err();
        assert!(matches!(err, ReceiveError::Closed));

        drop(senders);
        let err = receiver.recv().unwrap_err();
        assert!(matches!(err, ReceiveError::Closed));
    }

    #[test]
    fn senders_dropped() {
        let (sender, mut receiver) = channel::<Int>();
        let senders = repeat(sender).take(10).collect::<Vec<_>>();
        for i in 0..10 {
            senders[0].send(Int(i)).unwrap();
        }

        for i in 0..10 {
            assert_eq!(receiver.recv().unwrap().0, i);
        }

        let err = receiver.recv().unwrap_err();
        assert!(matches!(err, ReceiveError::Empty));
        drop(senders);

        let err = receiver.recv().unwrap_err();
        assert!(matches!(err, ReceiveError::Closed));
        assert!(Error::source(&err).is_none());

        receiver.close();
        let err = receiver.recv().unwrap_err();
        assert!(matches!(err, ReceiveError::Closed));
    }

    #[test]
    fn receiver_dropped() {
        let (sender, receiver) = channel::<Int>();
        let senders = repeat(sender).take(10).collect::<Vec<_>>();
        for i in 0..10 {
            senders[0].send(Int(i)).unwrap();
        }
        drop(receiver);

        for i in 0..10 {
            assert!(senders[i].is_closed());
            let err = senders[i].send(Int(i)).unwrap_err();
            assert!(Error::source(&err).is_none());
            assert_eq!(err.value.0, i);
        }
    }

    #[test]
    fn same_channel() {
        let (first, _) = channel::<Int>();
        assert!(first.same_channel(&first.clone()));

        let (second, _) = channel::<Int>();
        assert!(second.clone().same_channel(&second));
        assert!(!first.same_channel(&second));
        assert!(!second.same_channel(&first));
    }
}
