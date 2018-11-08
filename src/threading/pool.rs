use std::thread::Thread;

pub struct Pool {
    pub threads: Vec<Thread>
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            threads: Vec::new()
        }
    }

    pub fn push(&mut self, thread: Thread) {
        self.threads.push(thread)
    }
}

#[cfg(test)]
mod tests {
    use super::Pool;
    use std::thread;

    #[test]
    fn can_instantiate_pool() {
        let p = Pool::new();
    }

    #[test]
    fn can_push_thread() {
        let mut p = Pool::new();
        let th = thread::Thread::new(
            Option::from(String::from("test"))
        );
        thread::
        p.push(th);
    }
}
