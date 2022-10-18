fn main() {
    println!("Concurrency of Rust.")
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        sync::{mpsc, Arc, Barrier, Condvar, Mutex, Once},
        thread,
        time::Duration,
    };

    use thread_local::ThreadLocal;

    #[test]
    fn test_0() {
        let v = vec![1, 2, 3];
        let handel = thread::spawn(move || {
            println!("vec from outter: {:?}", v);
            for i in 1..10 {
                println!("hi number: {:?} from the spawn thread.", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // join() 让当前线程等待handel所代表的线程结束
        handel.join().unwrap();

        for i in 1..5 {
            println!("hi number: {:?} from the main thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn test_1() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    #[test]
    fn test_2() {
        let new_thread = thread::spawn(move || {
            thread::spawn(|| loop {
                println!("I'm a new thread")
            })
        });

        new_thread.join().unwrap();

        thread::sleep(Duration::from_millis(1000));
    }

    #[test]
    fn test_3() {
        // 线程屏障，类似于Java中的CountDownLatch
        let mut handles = Vec::with_capacity(6);
        let ba = Arc::new(Barrier::new(6));
        for _ in 0..6 {
            let b = Arc::clone(&ba);
            handles.push(thread::spawn(move || {
                println!("before wait");
                b.wait();
                println!("after wait");
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_4() {
        // FOO 是线程局部变量，即每个线程都会拥有一个FOO的副本
        thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
        });

        let t = thread::spawn(|| {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            });
        });

        t.join().unwrap();

        FOO.with(|f| {
            assert_eq!(*f.borrow(), 2);
        })
    }

    struct FOO;
    impl FOO {
        thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    }
    #[test]
    fn test_5() {
        FOO::FOO.with(|f| *f.borrow_mut() = 2);
    }

    #[test]
    fn test_6() {
        let tls = Arc::new(ThreadLocal::new());

        for _ in 0..5 {
            let t = Arc::clone(&tls);
            thread::spawn(move || {
                // 每个线程持有值的一份独立拷贝
                let cell = t.get_or(|| Cell::new(0));
                cell.set(cell.get() + 1);
            })
            .join()
            .unwrap();
        }

        let ts = Arc::try_unwrap(tls).unwrap();
        let total = ts.into_iter().fold(0, |x, y| x + y.get());

        assert_eq!(total, 5);
    }

    #[test]
    fn test_7() {
        // 用条件控制线程的挂起和执行
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        thread::spawn(move || {
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            println!("changing started");
            *started = true;
            cvar.notify_one();
        });

        let &(ref lock, ref cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }

        println!("started changed.")
    }

    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    #[test]
    fn test_8() {
        // 只能调用一次的函数
        let handle1 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 1;
            });
        });

        let handle2 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 2;
            });
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        println!("{}", unsafe { VAL });
        // VAL 的值是1或2，取决于哪一个线程先执行，无法确定
        assert!(unsafe { VAL == 1 || VAL == 2 })
    }

    #[test]
    fn test_9() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                "hi".to_string(),
                "from".to_string(),
                "the".to_string(),
                "thread".to_string(),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                "hi".to_string(),
                "from".to_string(),
                "the".to_string(),
                "thread".to_string(),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("{:?}", received)
        }
    }
}
