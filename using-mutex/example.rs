use std::thread;
use std::sync::{Arc, Mutex};

struct Point {
	x: f64,
	y: f64
}

fn main() {
	// Criando um ponteiro atômico para o Mutex que guarda um ponto
	let m = Arc::new(Mutex::new(Point{x:0.0, y:0.0}));

	// Clonamos nossa referência para evitar problema de ownership
	// Isso soma ao nosso Arc uma nova referência ao Mutex m
	let m_cloned = Arc::clone(&m);
	// Chama uma nova thread para mover o ponto 5u na diagonal (45°)
	let thread1 = thread::spawn(move || {
		let mut point = m_cloned.lock().unwrap();
		point.x += 3.0;
		point.y += 4.0;
	}); // Quando o código chegar aqui, o mutex será desbloqueado
	// Aqui o m_cloned não existe mais, pois foi movido para um outro escopo

	thread1.join().unwrap(); // espera a thread1 executar
	
	let point = m.lock().unwrap();
	println!("x = {}, y = {}", point.x, point.y);
}