function setupCameraEditListeners() {
    const playAndBackBtn = document.getElementById('play-and-back-btn');
    playAndBackBtn.addEventListener('click', () => {
        sendCommand('toggle_play');
        document.getElementById('page-camera-edit').style.display = 'none';
        document.getElementById('page-visualize').style.display = 'block';
        document.getElementById('main-title').textContent = 'VisuGPS Visualisation';
    });

    // Touch Areas
    const panArea = document.getElementById('pan-area');
    const bearingArea = document.getElementById('bearing-area');
    const zoomArea = document.getElementById('zoom-area');
    const tiltArea = document.getElementById('tilt-area');

    const handleDrag = (element, onDrag) => {
        let isDragging = false;
        let lastX = 0;
        let lastY = 0;

        const start = (x, y) => {
            isDragging = true;
            lastX = x;
            lastY = y;
            element.style.backgroundColor = '#555';
        };

        const move = (x, y) => {
            if (!isDragging) return;
            const dx = x - lastX;
            const dy = y - lastY;
            lastX = x;
            lastY = y;
            onDrag(dx, dy);
        };

        const end = () => {
            isDragging = false;
            element.style.backgroundColor = '#444';
        };

        // Mouse events
        element.addEventListener('mousedown', (e) => start(e.clientX, e.clientY));
        document.addEventListener('mousemove', (e) => move(e.clientX, e.clientY));
        document.addEventListener('mouseup', end);

        // Touch events
        element.addEventListener('touchstart', (e) => {
            e.preventDefault();
            const touch = e.touches[0];
            start(touch.clientX, touch.clientY);
        }, { passive: false });
        element.addEventListener('touchmove', (e) => {
            const touch = e.touches[0];
            move(touch.clientX, touch.clientY);
        });
        element.addEventListener('touchend', end);
        element.addEventListener('touchcancel', end);
    };

    handleDrag(panArea, (dx, dy) => {
        const panX = dx * g_sensibility_point_de_vue_x * -1;
        const panY = dy * g_sensibility_point_de_vue_y * -1;
        sendCommand('update_camera', { pan: [panX, panY] });
    });

    handleDrag(bearingArea, (dx, dy) => {
        const bearingDelta = dx * g_sensibility_cap * -1;
        sendCommand('update_camera', { bearing: bearingDelta });
    });

    handleDrag(zoomArea, (dx, dy) => {
        const zoomDelta = dy * g_sensibility_zoom * -1;
        sendCommand('update_camera', { zoom: zoomDelta });
    });

    handleDrag(tiltArea, (dx, dy) => {
        const tiltDelta = dy * g_sensibility_tilt * -1;
        sendCommand('update_camera', { pitch: tiltDelta });
    });
}