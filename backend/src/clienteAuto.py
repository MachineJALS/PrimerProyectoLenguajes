import socket, json, time

# Listas predefinidas de opciones
cantidades_asientos = [5, 2, 2, 7]
precios_maximos = [100, 130, 75, 102]

def enviar_solicitud(cantidad, precio_max, s):
    # Crear la solicitud
    solicitud = {
        "cantidad": cantidad,
        "precio_max": precio_max
    }

    # Convertir la solicitud a JSON y enviarla
    mensaje = json.dumps(solicitud) + '\n'
    s.sendall(mensaje.encode())

    # Recibir la respuesta del servidor
    response = s.recv(4096)  # Recibir hasta 4096 bytes
    asientos = response.decode()
    # Imprimir los asientos propuestos
    print(f"Asientos propuestos para {cantidad} asientos a un máximo de ${precio_max} por asiento:", asientos)

    # Automatizar la decisión (aceptar o rechazar secuencialmente)
    decision = 'aceptar' if len(cantidades_asientos) % 2 == 0 else 'rechazar'
    print(f"Decisión automatizada: {decision}")

    # Enviar la decisión al servidor
    s.sendall((decision + '\n').encode())

    # Recibir la confirmación del servidor
    confirmacion = s.recv(4096).decode()
    print("Confirmación del servidor:", confirmacion)

def main():
    print("Bienvenido al sistema TSOMachine. Realizando solicitudes automáticas...")

    # Dirección del servidor y puerto
    server_address = ('127.0.0.1', 8080)

    num_requests = min(len(cantidades_asientos), len(precios_maximos))  # Número de solicitudes a enviar

    for i in range(num_requests):
        cantidad = cantidades_asientos[i]
        precio_max = precios_maximos[i]

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            try:
                # Conectar al servidor
                s.connect(server_address)

                # Enviar la solicitud
                enviar_solicitud(cantidad, precio_max, s)

                # Espera entre solicitudes
                time.sleep(1)  # Espera 1 segundo entre solicitudes (ajustar si es necesario)

            except socket.error as e:
                print(f"Error de socket: {e}")

if __name__ == "__main__":
    main()
