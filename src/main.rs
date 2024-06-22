use std::sync::Arc;

use parking_lot::{RwLock, RwLockReadGuard};

#[derive(Clone)]
struct Person {
    inner: Arc<RwLock<PersonInner>>,
}

struct PersonInner {
    name: String,
    age: u32,
}

struct PersonReadGuard<'a> {
    guard: RwLockReadGuard<'a, PersonInner>,
}

impl Person {
    fn read(&self) -> PersonReadGuard {
        PersonReadGuard {
            guard: self.inner.read(),
        }
    }

    /// Returns the new age
    fn birthday(&self) -> u32 {
        let mut guard = self.inner.write();
        guard.age += 1;
        guard.age
    }
}

impl PersonReadGuard<'_> {
    fn can_access(&self) -> bool {
        const MIN_AGE: u32 = 18;

        self.guard.age >= MIN_AGE
    }

    fn get_name(&self) -> &String {
        &self.guard.name
    }
}

fn main() {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        for deadlock in parking_lot::deadlock::check_deadlock() {
            for deadlock in deadlock {
                println!(
                    "Found a deadlock! {}:\n{:?}",
                    deadlock.thread_id(),
                    deadlock.backtrace()
                );
            }
        }
    });

    let alice = Person {
        inner: Arc::new(RwLock::new(PersonInner {
            name: "Alice".to_owned(),
            age: 15,
        })),
    };

    let alice_clone = alice.clone();
    std::thread::spawn(move || loop {
        let guard = alice_clone.read();
        println!("Downloading a cute loading image, please wait...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!(
            "Does the {} have access? {}",
            guard.get_name(),
            guard.can_access()
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let new_age = alice.birthday();

        println!("Happy birthday! Person is now {new_age} years old.");
    }
}
