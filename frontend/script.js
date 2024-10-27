// Seleccionar elementos del DOM
const reservaForm = document.getElementById("reserva-form");
const pagoForm = document.getElementById("pago-form");
const resultado = document.getElementById("resultado");

// Función para enviar la solicitud de reserva
async function solicitarReserva(event) {
    event.preventDefault();

    const cantidad = document.getElementById("cantidad").value;
    const precioMax = document.getElementById("precio_max").value;

    const solicitud = {
        cantidad: parseInt(cantidad),
        precio_max: parseFloat(precioMax),
    };

    try {
        const response = await fetch("http://192.168.10.3:8080", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(solicitud),
        });

        const data = await response.text();
        resultado.innerText = `Reserva: ${data}`;
    } catch (error) {
        resultado.innerText = "Error al solicitar reserva.";
    }
}

// Función para realizar el pago
async function realizarPago(event) {
    event.preventDefault();

    const tarjeta = document.getElementById("tarjeta").value;

    if (!/^\d{16}$/.test(tarjeta)) {
        resultado.innerText = "Número de tarjeta inválido. Debe contener 16 dígitos.";
        return;
    }

    try {
        const response = await fetch("http://192.168.10.3:8080", {
            method: "POST",
            headers: {
                "Content-Type": "text/plain",
            },
            body: tarjeta,
        });

        const data = await response.text();
        resultado.innerText = `Pago: ${data}`;
    } catch (error) {
        resultado.innerText = "Error al realizar el pago.";
    }
}

// Eventos para enviar formularios
reservaForm.addEventListener("submit", solicitarReserva);
pagoForm.addEventListener("submit", realizarPago);
