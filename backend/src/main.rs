use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum EstadoAsiento {
    Libre,
    Reservado,
    Comprado,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Asiento {
    seccion: char,
    fila: u8,
    numero: u8,
    visibilidad: u8,
    vip: bool,
    precio: f64,
    estado: EstadoAsiento,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Seccion {
    nombre: char,
    asientos: Vec<Asiento>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Solicitud {
    cantidad: usize,
    precio_max: f64,
}

// Función para ocupar asientos de forma aleatoria
fn ocupar_asientos_aleatoriamente(
    seccion: &mut Seccion,
    cantidad: usize,
) {
    // Filtrar asientos disponibles
    let mut asientos_disponibles: Vec<&mut Asiento> = seccion
        .asientos
        .iter_mut()
        .filter(|asiento| asiento.estado == EstadoAsiento::Libre)
        .collect();

    // Mezclar y seleccionar los asientos
    let mut rng = rand::thread_rng();
    asientos_disponibles.shuffle(&mut rng);
    let seleccionados = asientos_disponibles.into_iter().take(cantidad);

    // Marcar los asientos seleccionados como reservados
    for asiento in seleccionados {
        asiento.estado = EstadoAsiento::Comprado;
    }
}

// Función para verificar los estados de todos los asientos antes de cualquier acción
fn verificar_asientos(secciones: &HashMap<char, Seccion>) {
    for seccion in secciones.values() {
        println!("Verificando asientos en la sección: {}", seccion.nombre);
        mostrar_estados_seccion(seccion);
    }
}

#[tokio::main]
async fn main() {
    // Inicializar las secciones y asientos
    let secciones = Arc::new(Mutex::new(inicializar_secciones()));

    let mut secciones_guardadas = secciones.clone();
    //Ocupar asientos de la seccion A,B,C y D de forma aleatoria
    {
        let mut secciones_guardadas = secciones_guardadas.lock().await;
        ocupar_asientos_aleatoriamente(&mut secciones_guardadas.get_mut(&'A').unwrap(), 5);
        ocupar_asientos_aleatoriamente(&mut secciones_guardadas.get_mut(&'B').unwrap(), 5);
        ocupar_asientos_aleatoriamente(&mut secciones_guardadas.get_mut(&'C').unwrap(), 5);
        ocupar_asientos_aleatoriamente(&mut secciones_guardadas.get_mut(&'D').unwrap(), 5);
    }
    
    // Verificar asientos antes de iniciar el servidor
    {
        let secciones_guardadas = secciones.lock().await;
        verificar_asientos(&secciones_guardadas);
    }

    //Cambiar la ip a la ip de la maquina 
    //ipconfing
    let listener = TcpListener::bind("192.168.1.1:8080").await.unwrap();
    println!("Servidor escuchando en 192.168.1.1:8080");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let secciones = Arc::clone(&secciones);
        tokio::spawn(handle_client(socket, secciones));
    }
}

async fn handle_client(stream: tokio::net::TcpStream, secciones: Arc<Mutex<HashMap<char, Seccion>>>) {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();

    // Leer la solicitud del cliente
    match reader.read_line(&mut buffer).await {
        Ok(_) => {
            // Parsear la solicitud
            let solicitud: Solicitud = match serde_json::from_str(&buffer) {
                Ok(s) => s,
                Err(_) => {
                    println!("Error al parsear la solicitud.");
                    return;
                }
            };
            println!("Solicitud recibida: {:?}", solicitud);

            let mut secciones = secciones.lock().await;

            // Verificar asientos antes de la reserva
            println!("Estado de los asientos antes de la reserva:");
            verificar_asientos(&secciones);
            
            // Buscar los mejores asientos
            let mut mejores_asientos = buscar_mejores_asientos(&secciones, solicitud.cantidad, solicitud.precio_max);
            let mut response = String::new();

            if let Some(asientos) = &mejores_asientos {
                // Marcar asientos como reservados
                for asiento in asientos.iter() {
                    if let Some(seccion) = secciones.get_mut(&asiento.seccion) {
                        if let Some(asiento_seleccionado) = seccion.asientos.iter_mut().find(|a| a.fila == asiento.fila && a.numero == asiento.numero) {
                            asiento_seleccionado.estado = EstadoAsiento::Reservado;
                        }
                    }
                }
                response = serde_json::to_string(&asientos).unwrap();
            } else {
                response = "No se encontraron asientos que cumplan los criterios.".to_string();
            }

            // Enviar la respuesta al cliente
            if let Err(e) = reader.get_mut().write_all(response.as_bytes()).await {
                eprintln!("Error al enviar la respuesta: {}", e);
                return;
            }

            // Leer la respuesta del cliente
            buffer.clear();
            match reader.read_line(&mut buffer).await {
                Ok(_) => {
                    let decision = buffer.trim().to_lowercase();
                    let confirmacion = match decision.as_str() {
                        "aceptar" => {
                            // Actualizar los asientos a comprados
                            if let Some(asientos) = &mejores_asientos {
                                for asiento in asientos {
                                    if let Some(seccion) = secciones.get_mut(&asiento.seccion) {
                                        if let Some(asiento_seleccionado) = seccion.asientos.iter_mut().find(|a| a.fila == asiento.fila && a.numero == asiento.numero) {
                                            asiento_seleccionado.estado = EstadoAsiento::Comprado;
                                        }
                                    }
                                }
                            }
                            "Asientos comprados exitosamente.".to_string()
                        },
                        "rechazar" => {
                            // Actualizar los asientos a libres
                            if let Some(asientos) = &mejores_asientos {
                                for asiento in asientos {
                                    if let Some(seccion) = secciones.get_mut(&asiento.seccion) {
                                        if let Some(asiento_seleccionado) = seccion.asientos.iter_mut().find(|a| a.fila == asiento.fila && a.numero == asiento.numero) {
                                            asiento_seleccionado.estado = EstadoAsiento::Libre;
                                        }
                                    }
                                }
                            }
                            "Asientos liberados.".to_string()
                        },
                        _ => "Opción no válida.".to_string(),
                    };

                    if let Err(e) = reader.get_mut().write_all(confirmacion.as_bytes()).await {
                        eprintln!("Error al enviar la confirmación: {}", e);
                    }

                    // Verificar asientos después de la acción del cliente
                    println!("Estado de los asientos después de la acción del cliente:");
                    verificar_asientos(&secciones);
                }
                Err(e) => {
                    eprintln!("Error al leer la decisión del cliente: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error al leer la solicitud: {}", e);
        }
    }
}

// Función para inicializar las secciones
fn inicializar_secciones() -> HashMap<char, Seccion> {
    let mut secciones = HashMap::new();
    secciones.insert('A', inicializar_seccion('A'));
    secciones.insert('B', inicializar_seccion('B'));
    secciones.insert('C', inicializar_seccion('C'));
    secciones.insert('D', inicializar_seccion('D'));
    secciones
}

// Función para inicializar una sección con sus asientos
fn inicializar_seccion(nombre: char) -> Seccion {
    let mut asientos = Vec::new();
    
    for fila in 1..=2 {
        for numero in 1..=12 {
            let visibilidad = match numero {
                1 | 12 => 50,   // Ejemplo: visibilidad más baja
                2 | 11 => 70,
                3 | 10 => 85,
                _ => 100,      // Ejemplo: visibilidad más alta
            };
            let vip = es_vip(fila, numero);
            let precio = calcular_precio(visibilidad, vip);

            asientos.push(Asiento {
                seccion: nombre,
                fila,
                numero,
                visibilidad,
                vip,
                precio,
                estado: EstadoAsiento::Libre,
            });
        }
    }

    Seccion { nombre, asientos }
}

// Determina si un asiento es VIP
fn es_vip(fila: u8, numero: u8) -> bool {
    fila == 1 && (numero == 1 || numero == 2 || numero == 11 || numero == 12)
}

// Calcula el precio del asiento basado en la visibilidad y si es VIP
fn calcular_precio(visibilidad: u8, vip: bool) -> f64 {
    let base_price = 50.0 + (visibilidad as f64 * 0.5);
    if vip {
        base_price * 1.5
    } else {
        base_price
    }
}

// Motor de búsqueda de asientos
fn buscar_mejores_asientos(secciones: &HashMap<char, Seccion>, cantidad: usize, precio_max: f64) -> Option<Vec<Asiento>> {
    let mut mejores_opciones: Option<Vec<Asiento>> = None;

    for seccion in secciones.values() {
        // Filtrar asientos disponibles en la sección
        let mut asientos_candidatos: Vec<Asiento> = seccion
            .asientos
            .iter()
            .filter(|asiento| asiento.estado == EstadoAsiento::Libre && asiento.precio <= precio_max)
            .cloned()
            .collect();

        // Ordenar los asientos candidatos por fila y número para proximidad
        asientos_candidatos.sort_by(|a, b| {
            a.fila.cmp(&b.fila)
                .then_with(|| a.numero.cmp(&b.numero))
                .then_with(|| b.visibilidad.cmp(&a.visibilidad))
        });

        // Intentar encontrar asientos consecutivos
        for i in 0..=(asientos_candidatos.len().saturating_sub(cantidad)) {
            let grupo = &asientos_candidatos[i..i + cantidad];
            let es_consecutivo = grupo.windows(2).all(|w| w[0].fila == w[1].fila && w[0].numero + 1 == w[1].numero);

            if es_consecutivo {
                mejores_opciones = Some(grupo.to_vec());
                break;
            }
        }
    }

    mejores_opciones
}

// Función para mostrar los estados de los asientos en una sección
fn mostrar_estados_seccion(seccion: &Seccion) {
    println!("Sección: {}", seccion.nombre);
    for asiento in &seccion.asientos {
        println!(
            "Fila: {}, Número: {}, Estado: {:?}, Precio: {:.2}",
            asiento.fila, asiento.numero, asiento.estado, asiento.precio
        );
    }
}

