import { ref } from 'vue';

export function useCameraManager(map) {
    // État de la caméra pour la pause/reprise
    const pausedCameraOptions = ref(null);
    const cameraMovedDuringPause = ref(false);
    const currentCameraBearing = ref(0);

    const saveCameraState = () => {
        if (!map.value) return;
        pausedCameraOptions.value = {
            center: map.value.getCenter(),
            zoom: map.value.getZoom(),
            pitch: map.value.getPitch(),
            bearing: map.value.getBearing(),
        };
        cameraMovedDuringPause.value = false;
    };

    const restoreCameraState = (duration = 2000) => {
        return new Promise((resolve) => {
            if (!map.value || !pausedCameraOptions.value) {
                resolve();
                return;
            }

            map.value.flyTo({
                ...pausedCameraOptions.value,
                duration: duration,
                easing: (t) => t, // linear
            });

            map.value.once('moveend', () => {
                resolve();
            });
        });
    };

    const enableInteraction = (callbacks = {}) => {
        if (!map.value) return;
        map.value.interactive = true;
        map.value.dragRotate.enable();
        map.value.dragPan.enable();
        map.value.scrollZoom.enable({ around: 'center' });

        // Attach listeners if provided
        if (callbacks.onMove) map.value.on('move', callbacks.onMove);
        if (callbacks.onZoom) map.value.on('zoom', callbacks.onZoom);
        if (callbacks.onPitch) map.value.on('pitch', callbacks.onPitch);
        if (callbacks.onRotate) map.value.on('rotate', callbacks.onRotate);
    };

    const disableInteraction = (callbacks = {}) => {
        if (!map.value) return;
        map.value.interactive = false;
        map.value.dragRotate.disable();
        map.value.dragPan.disable();
        map.value.scrollZoom.disable();

        // Detach listeners
        if (callbacks.onMove) map.value.off('move', callbacks.onMove);
        if (callbacks.onZoom) map.value.off('zoom', callbacks.onZoom);
        if (callbacks.onPitch) map.value.off('pitch', callbacks.onPitch);
        if (callbacks.onRotate) map.value.off('rotate', callbacks.onRotate);
    };

    // Suivi automatique du bearing
    const startBearingTracking = () => {
        if (!map.value) return;
        map.value.on('move', () => {
            currentCameraBearing.value = map.value.getBearing();
        });
    };

    return {
        pausedCameraOptions,
        cameraMovedDuringPause,
        currentCameraBearing,
        saveCameraState,
        restoreCameraState,
        enableInteraction,
        disableInteraction,
        startBearingTracking
    };
}
