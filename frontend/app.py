from flask import Flask, jsonify, request, render_template
import json
import os
import socket

app = Flask(__name__, static_folder='static', template_folder='templates')

ASIENTOS_FILE = os.path.join(os.path.dirname(__file__), 'asientos.json')
SERVER_ADDRESS = ('192.168.10.26', 8080)

server_socket = None

def initialize_server_connection():
    global server_socket
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.connect(SERVER_ADDRESS)
    print(f"Conexión establecida con {SERVER_ADDRESS}")

def close_server_connection():
    global server_socket
    if server_socket:
        server_socket.close()

@app.before_request
def setup_connection():
    global server_socket
    if server_socket is None:
        initialize_server_connection()

@app.route('/')
def index():
    return render_template('compra.html')

@app.route('/estado')
def estado():
    return render_template('estado.html')

@app.route('/estados.json')
def obtener_estados():
    with open(ASIENTOS_FILE) as f:
        asientos = json.load(f)
    return jsonify(asientos)

@app.route('/api/comprar', methods=['POST'])
def comprar():
    global server_socket
    data = request.get_json()
    cantidad = data.get('cantidad')
    precio_max = data.get('precio_maximo')
    solicitud = {
        "cantidad": cantidad,
        "precio_max": precio_max
    }
    try:
        mensaje = json.dumps(solicitud) + '\n'
        server_socket.sendall(mensaje.encode())
        response = server_socket.recv(4096).decode()
        return jsonify({
            "status": "success",
            "mensaje": "Asientos asignados temporalmente. Acepta o rechaza.",
            "asientos_asignados": response
        })
    except socket.error as e:
        return jsonify({"error": f"Error de conexión: {e}"}), 500

@app.route('/api/confirmar', methods=['POST'])
def confirmar():
    global server_socket
    data = request.get_json()
    try:
        server_socket.sendall((data.get('decision') + '\n').encode())
        confirmacion = server_socket.recv(4096).decode()
        return jsonify({"status": "success", "mensaje": confirmacion})
    except socket.error as e:
        return jsonify({"error": f"Error de conexión: {e}"}), 500

if __name__ == '__main__':
    try:
        initialize_server_connection()
        app.run(debug=True, port=8000, host='0.0.0.0')
    finally:
        close_server_connection()
