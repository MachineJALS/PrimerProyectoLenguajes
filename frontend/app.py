from flask import Flask, render_template, request, jsonify
from flask_sockets import Sockets

app = Flask(__name__)
sockets = Sockets(app)

# Ruta principal para cargar la interfaz de solicitud de asientos
@app.route('/')
def home():
    return render_template('index.html')

# Ruta para la página de pago
@app.route('/pago')
def pago():
    return render_template('realizar_pagos.html')

# Ruta para la página de estado de asientos
@app.route('/estado')
def estado():
    return render_template('estado_asientos.html')

# WebSocket para actualizaciones en tiempo real del estado de los asientos
@sockets.route('/ws')
def ws_estados(ws):
    while not ws.closed:
        # Simulación de actualización de estado de los asientos
        data = [
            {'Seccion':'A','fila': 1, 'columna': 1, 'estado': 'libre'},
            {'Seccion':'B','fila': 1, 'columna': 2, 'estado': 'reservado'},
            {'Seccion':'C','fila': 1, 'columna': 3, 'estado': 'ocupado'}
            # Puedes agregar más asientos aquí según la necesidad
        ]
        ws.send(jsonify(data))
        ws.receive()  # Mantener el WebSocket abierto y esperando mensajes

if __name__ == "__main__":
    app.run(debug=True)
