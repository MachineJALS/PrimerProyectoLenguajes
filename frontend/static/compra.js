document.getElementById('reserva-form').addEventListener('submit', async function (event) {
    event.preventDefault();

    const cantidad = document.getElementById('cantidad').value;
    const precioMax = document.getElementById('precio-max').value;

    const solicitud = { cantidad: parseInt(cantidad), precio_max: parseFloat(precioMax) };

    try {
        const response = await fetch('/api/comprar', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(solicitud),
        });

        const data = await response.json();
        document.getElementById('resultado').innerText = JSON.stringify(data, null, 2);
    } catch (error) {
        console.error("Error al realizar la solicitud:", error);
    }
});
