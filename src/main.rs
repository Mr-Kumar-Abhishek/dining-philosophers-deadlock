use std::thread;
use std::sync::{Mutex, Arc};
struct Philosopher {
	name: String,
	left: usize,
	right: usize,
}

impl Philosopher {
	fn new(name: &str, left: usize, right: usize) -> Philosopher {
		Philosopher {
			name: name.to_string(),
			left: left,
			right: right,
		}
	}
	
	fn eat(&self, table: &Table) {
		let _left = table.forks[self.left].lock().unwrap();
		let _right = table.forks[self.right].lock().unwrap();

		println!("{} is eating.", self.name);
		thread::sleep_ms(1000);
		println!("{} has done eating.", self.name);
	}
}

struct Table {
	forks: Vec<Mutex<()>>,
}

fn main(){
	let table = Arc::new(Table { forks: vec![
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
	]});

	let philosophers = vec![
		Philosopher::new("Acharya Vamana", 0, 1),
		Philosopher::new("Maha Kapphina", 1, 2),
		Philosopher::new("Gargi Vachaknavi", 2, 3),
		Philosopher::new("Vacaspati Misra", 3, 4),
		Philosopher::new("Adi Shankra", 0, 4),
	];
	
	let handle: Vec<_> = philosophers.into_iter().map(|p| {
		
		let table = table.clone();

		thread::spawn(move || {
			p.eat(&table);
		})
	}).collect();

	for h in handle {
		h.join().unwrap();
	}
}

