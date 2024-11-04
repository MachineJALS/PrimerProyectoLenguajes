document.getElementById("pago-form").addEventListener("submit", async (event) => {
    event.preventDefault();

    const tarjeta = document.getElementById("tarjeta").value;
    const cvv = document.getElementById("cvv").value;

    if (!/^\d{16}$/.test(tarjeta)) {
        document.getElementById("resultado").innerText = "Número de tarjeta inválido. Debe contener 16 dígitos.";
        return;
    }

    if (!/^\d{3,4}$/.test(cvv)) {
        document.getElementById("resultado").innerText = "CVV inválido. Debe contener 3 o 4 dígitos.";
        return;
    }

    try {
        const response = await fetch("http://192.168.10.3:8080/pagar", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ tarjeta, cvv }),
        });
        const data = await response.text();
        document.getElementById("resultado").innerText = `Pago: ${data}`;
    } catch (error) {
        document.getElementById("resultado").innerText = "Error al realizar el pago.";
    }
});
