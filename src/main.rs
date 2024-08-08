mod commands;
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    println!("Servidor escuchando en 127.0.0.1:6379");

    let storage = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nueva conexión establecida!");
                let storage_clone = Arc::clone(&storage);
                thread::spawn(move || handle_conection(stream, storage_clone));
            }
            Err(e) => eprintln!("Error al aceptar la conexión: {}", e),
        }
    }

    Ok(())
}

fn handle_conection(mut stream: TcpStream, storage: Arc<Mutex<HashMap<String, String>>>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    println!("Conexion cerrada por el cliente.");
                    break;
                }
                let received = String::from_utf8_lossy(&buffer[..size]);
                let parts: Vec<&str> = received.split_whitespace().collect();
                match parts[0] {
                    "GET" => {
                        if parts.len() < 2 {
                            println!("SET requiere una clave");
                            continue;
                        }
                        let key = &parts[1];
                        match commands::get(&storage, key) {
                            Some(value) => {
                                let response = format!("VALUE: {}\n", value);
                                println!("{}", response);
                                if let Err(e) = stream.write(response.as_bytes()) {
                                    eprintln!("Error al escribir en el stream:{}", e)
                                }
                            }
                            None => {
                                let _ = stream.write(b"Clave no encontrada\n");
                            }
                        }
                    }
                    "SET" => {
                        if parts.len() < 3 {
                            println!("SET requiere una clave y un valor");
                            continue;
                        }
                        let key = parts[1].to_string();
                        let value = parts[2].to_string();
                        commands::set(&storage, key, value);
                    }
                    _ => {
                        println!("Comando no conocido")
                    }
                }

                if let Err(e) = stream.write(b"Ok\n") {
                    eprintln!("Error al enviar respuesta: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("Error al leer el stream: {}", e);
                break;
            }
        }
    }
}
