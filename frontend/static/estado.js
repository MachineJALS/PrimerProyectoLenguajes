async function cargarEstadoEstadio() {
    try {
        const response = await fetch('/estados.json');
        const asientos = await response.json();

        const svgContainer = document.getElementById('svg-container');
        svgContainer.innerHTML = '';

        const svgNS = "http://www.w3.org/2000/svg";
        const svg = document.createElementNS(svgNS, "svg");
        svg.setAttribute('width', 600);
        svg.setAttribute('height', 400);

        // Dibujar los asientos por secciones
        asientos.forEach(asiento => {
            const rect = document.createElementNS(svgNS, "rect");

            // Posiciones calculadas con filas, columnas y sección
            const x = (asiento.columna - 1) * 50 + (asiento.Seccion.charCodeAt(0) - 65) * 200; // Letra de la sección en ASCII
            const y = (asiento.fila - 1) * 50;

            rect.setAttribute('x', x);
            rect.setAttribute('y', y);
            rect.setAttribute('width', 40);
            rect.setAttribute('height', 40);

            // Asignar clase según el estado del asiento
            rect.setAttribute('class', `asiento ${asiento.estado}`);
            svg.appendChild(rect);
        });

        svgContainer.appendChild(svg);
    } catch (error) {
        console.error("Error al cargar el estado del estadio:", error);
    }
}

cargarEstadoEstadio();
