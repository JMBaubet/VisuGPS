import { ref, shallowRef } from 'vue';
import mapboxgl from 'mapbox-gl';

export function useMapEngine(mapContainer, mapboxToken, mapStyle, terrainExaggeration) {
    const map = shallowRef(null);
    const isMapLoaded = ref(false);

    // Fonction flyTo sous forme de Promise pour séquencer les animations
    const flyToPromise = (options) => {
        return new Promise((resolve) => {
            if (!map.value) {
                resolve();
                return;
            }
            map.value.flyTo({
                ...options,
                essential: true, // ensure authentication
            });
            map.value.once('moveend', () => {
                resolve();
            });
        });
    };

    const initializeMap = async (center, zoom) => {
        if (!mapboxToken.value || !mapContainer.value) {
            console.error("Map Init Aborted:", { token: !!mapboxToken.value, container: !!mapContainer.value, containerRef: mapContainer.value });
            return;
        }
        mapboxgl.accessToken = mapboxToken.value;

        return new Promise((resolve) => {
            map.value = new mapboxgl.Map({
                container: mapContainer.value,
                style: mapStyle.value,
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
            });

            map.value.on('load', () => {
                isMapLoaded.value = true;
                resolve(map.value);
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
