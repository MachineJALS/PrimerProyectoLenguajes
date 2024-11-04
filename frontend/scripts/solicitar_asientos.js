document.getElementById("reserva-form").addEventListener("submit", async (event) => {
    event.preventDefault();
    const cantidad = document.getElementById("cantidad").value;
    const precioMax = document.getElementById("precio_max").value;

    const solicitud = {
        cantidad: parseInt(cantidad),
        precio_max: parseFloat(precioMax),
    };

    try {
        const response = await fetch("http://192.168.10.3:8080/solicitar", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(solicitud),
        });
        const data = await response.text();
        document.getElementById("resultado").innerText = `Reserva: ${data}`;
    } catch (error) {
        document.getElementById("resultado").innerText = "Error al solicitar reserva.";
    }
});
