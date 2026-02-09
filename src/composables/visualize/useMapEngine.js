import { ref, shallowRef } from 'vue';
import mapboxgl from 'mapbox-gl';

export function useMapEngine(mapContainer, mapboxToken, mapStyle, terrainExaggeration) {
    const map = shallowRef(null);
    const isMapLoaded = ref(false);

    // Fonction flyTo sous forme de Promise pour séquencer les animations
    const flyToPromise = (options, overrideOptions = {}) => {
        return new Promise((resolve) => {
            if (!map.value) {
                resolve();
                return;
            }

            const combinedOptions = { ...options, ...overrideOptions };
            const duration = combinedOptions.duration || 0;
            let resolved = false;

            const safeResolve = () => {
                if (!resolved) {
                    resolved = true;
                    if (timeout) clearTimeout(timeout);
                    map.value.off('moveend', onMoveEnd);
                    resolve();
                }
            };

            const onMoveEnd = () => {
                safeResolve();
            };

            // Attach listener BEFORE calling flyTo to avoid race condition
            map.value.once('moveend', onMoveEnd);

            // Safety timeout: duration + 500ms margin (minimum 1000ms safety)
            const timeout = setTimeout(() => {
                console.warn("[flyToPromise] Safety timeout reached. Forcing resolution.");
                safeResolve();
            }, Math.max(duration + 500, 1000));

            try {
                map.value.flyTo(combinedOptions);
            } catch (err) {
                console.error("[flyToPromise] flyTo error:", err);
                safeResolve();
            }
        });
    };

    const initializeMap = async (center, zoom, styleOverride = null) => {
        if (!mapboxToken.value || !mapContainer.value) {
            console.error("Map Init Aborted:", { token: !!mapboxToken.value, container: !!mapContainer.value, containerRef: mapContainer.value });
            return;
        }
        mapboxgl.accessToken = mapboxToken.value;

        return new Promise((resolve) => {
            map.value = new mapboxgl.Map({
                container: mapContainer.value,
                style: styleOverride || mapStyle.value,
                center: center,
                zoom: zoom,
                pitch: 0,
                bearing: 0,
                interactive: false,
            });

            map.value.on('style.load', () => {
                // Configuration standard (Terrain, Fog)
                if (!map.value.getSource('mapbox-dem')) {
                    map.value.addSource('mapbox-dem', {
                        'type': 'raster-dem',
                        'url': 'mapbox://mapbox.mapbox-terrain-dem-v1',
                        'tileSize': 512,
                        'maxzoom': 14
                    });
                }
                map.value.setTerrain({ 'source': 'mapbox-dem', 'exaggeration': terrainExaggeration?.value || 1.5 });
                map.value.setFog({});

                // Resolve earlier to show Europe tiles loading instead of black screen
                resolve(map.value);
            });

            map.value.on('load', () => {
                isMapLoaded.value = true;
                // isMapLoaded remains for layers logic
            });
        });
    };

    const cleanupMap = () => {
        if (map.value) {
            map.value.remove();
            map.value = null;
        }
    };

    return {
        map,
        isMapLoaded,
        initializeMap,
        flyToPromise,
        cleanupMap
    };
}
