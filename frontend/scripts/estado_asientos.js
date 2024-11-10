const socket = new WebSocket("ws://192.168.10.3:8080/ws");

socket.addEventListener("open", () => {
    console.log("Conectado al servidor de WebSocket");
});

socket.addEventListener("message", (event) => {
    const data = JSON.parse(event.data);
    const asientosContainer = document.getElementById("asientos-container");
    asientosContainer.innerHTML = ""; // Limpiar asientos previos

    data.forEach(asiento => {
        const asientoDiv = document.createElement("div");
        asientoDiv.className = "asiento";
        
        // Aplicar color según el estado del asiento
        asientoDiv.classList.add(asiento.estado); // libre, reservado, ocupado
        asientoDiv.innerText = `${asiento.fila}-${asiento.columna}`;

        // Añadir interactividad para cambiar estado localmente
        asientoDiv.addEventListener("click", () => {
            if (asientoDiv.classList.contains("libre")) {
                asientoDiv.classList.remove("libre");
                asientoDiv.classList.add("reservado");
                asientoDiv.style.backgroundColor = "#ffd54f";
            } else if (asientoDiv.classList.contains("reservado")) {
                asientoDiv.classList.remove("reservado");
                asientoDiv.classList.add("libre");
                asientoDiv.style.backgroundColor = "#81c784";
            }
        });

        asientosContainer.appendChild(asientoDiv);
    });
});

socket.addEventListener("close", () => {
    console.log("Desconectado del servidor de WebSocket");
});
