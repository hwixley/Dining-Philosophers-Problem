use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

struct Fork;

struct Philosopher {
    index: usize,
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
    eat_count: Arc<Mutex<Vec<usize>>>,
}

impl Philosopher {
    fn think(&self) {
        let mut rng = rand::rng();
        let delay = rng.random_range(10..100);
        thread::sleep(Duration::from_millis(delay));
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        println!("{} is trying to eat", &self.name);
        let _left = self.left_fork.lock().unwrap();
        println!("{} picked up left fork", &self.name);
        let _right = self.right_fork.lock().unwrap();
        println!("{} picked up right fork", &self.name);

        let mut table = "".to_string();
        table.push_str(&"💭".repeat(self.index));
        table.push_str("🍝");
        table.push_str(&"💭".repeat(4-self.index));
        println!("\n{}", table);
        println!("{} is eating...", &self.name);
        let mut rng = rand::rng();
        let delay = rng.random_range(10..100);
        thread::sleep(Duration::from_millis(delay));

        let mut counts = self.eat_count.lock().unwrap();
        counts[self.index] += 1;

        println!("{} has finished eating and released forks", &self.name);
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    let (tx, rx) = mpsc::sync_channel(10);

    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    let eat_count = Arc::new(Mutex::new(vec![0; PHILOSOPHERS.len()]));

    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for i in 0..forks.len() {
        let tx = tx.clone();
        let mut left_fork = Arc::clone(&forks[i]);
        let mut right_fork = Arc::clone(&forks[(i + 1) % forks.len()]);

        // Break symmetry to avoid deadlock
        if i == forks.len() - 1 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }

        let eat_count = Arc::clone(&eat_count);
        let philosopher = Philosopher {
            index: i,
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork,
            eat_count
        };

        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            for _ in 0..100 {
                philosopher.eat();
                philosopher.think();
            }
        });
        handles.push(handle);
    }

    drop(tx);
    for thought in rx {
        println!("{thought}");
    }

    for handle in handles {
        handle.join().unwrap(); // Ensure all threads complete execution
    }

    // Print summary statistics
    let counts = eat_count.lock().unwrap();
    println!("\n📊 Eating Statistics:");
    let total_meals: usize = counts.iter().sum();
    let max_eater = counts.iter().enumerate().max_by_key(|&(_, &c)| c).unwrap();
    let min_eater = counts.iter().enumerate().min_by_key(|&(_, &c)| c).unwrap();

    for (i, &count) in counts.iter().enumerate() {
        println!("🍽️ {} ate {} times.", PHILOSOPHERS[i], count);
    }

    println!("\n🍽️ Total meals eaten: {}", total_meals);
    println!(
        "🥇 Most meals: {} with {} meals",
        PHILOSOPHERS[max_eater.0], max_eater.1
    );
    println!(
        "🥉 Least meals: {} with {} meals",
        PHILOSOPHERS[min_eater.0], min_eater.1
    );

    println!("🍽️ All philosophers are done eating.");
}