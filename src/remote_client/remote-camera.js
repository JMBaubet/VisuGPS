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
        sendCommand('update_camera', { type: 'pan', dx: dx, dy: dy });
    });

    handleDrag(bearingArea, (dx, dy) => {
        sendCommand('update_camera', { type: 'bearing', dx: dx, dy: dy });
    });

    handleDrag(zoomArea, (dx, dy) => {
        sendCommand('update_camera', { type: 'zoom', dx: dx, dy: dy });
    });

    handleDrag(tiltArea, (dx, dy) => {
        sendCommand('update_camera', { type: 'tilt', dx: dx, dy: dy });
    });
}