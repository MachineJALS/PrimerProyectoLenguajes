document.getElementById("actualizar-estado").addEventListener("click", async () => {
    try {
        const response = await fetch("http://192.168.10.3:8080/estado", {
            method: "GET",
        });
        const data = await response.json();
        const asientosContainer = document.getElementById("asientos-container");
        asientosContainer.innerHTML = ""; // Limpiar asientos previos

        data.asientos.forEach(asiento => {
            const asientoDiv = document.createElement("div");
            asientoDiv.className = "asiento";
            asientoDiv.classList.add(asiento.estado); // Añade la clase `libre` o `reservado`
            asientoDiv.innerText = `${asiento.fila}-${asiento.columna}`;
            asientosContainer.appendChild(asientoDiv);
        });
    } catch (error) {
        document.getElementById("estado").innerText = "Error al obtener el estado de los asientos.";
    }
});
