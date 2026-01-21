import { ref, shallowRef } from 'vue';

export function useTraceLayers(map) {
    const coloredSegmentsGeoJsonRef = shallowRef(null);

    // Configuration par défaut, pourra être surchargée
    const config = {
        traceWidth: 4,
        traceOpacity: 1,
        traceColor: '#0000FF',
        cometWidth: 5,
        cometColor: '#FF0000',
        cometOpacity: 0.8
    };

    const setupTraceLayers = (configs = {}) => {
        if (!map.value) return;

        const {
            traceWidth, traceOpacity, traceColor,
            lineStringData, coloredSegmentsData,
            cometWidth, cometColor, cometOpacity
        } = { ...config, ...configs };

        if (coloredSegmentsData) {
            coloredSegmentsGeoJsonRef.value = coloredSegmentsData;
        }

        // 1. Source Trace Principale
        if (!map.value.getSource('trace') && lineStringData) {
            map.value.addSource('trace', { type: 'geojson', data: lineStringData, lineMetrics: true });
        }

        // 2. Source Segments Colorés
        if (coloredSegmentsGeoJsonRef.value && !map.value.getSource('colored-segments')) {
            map.value.addSource('colored-segments', { type: 'geojson', data: coloredSegmentsGeoJsonRef.value });
        }

        // 3. Layer Trace Complete (Fallback)
        // Logique "Revert" intégrée : On utilise trace-complete si pas de segments, 
        // ou si on veut une 'backtrace' unifiée
        if (!map.value.getLayer('trace-complete')) {
            if (map.value.getSource('trace')) {
                map.value.addLayer({
                    id: 'trace-complete',
                    type: 'line',
                    source: 'trace',
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                    paint: {
                        'line-width': traceWidth,
                        'line-opacity': traceOpacity,
                        'line-color': traceColor
                    }
                });
            }
        }

        // 4. Layers Overlap (Si segments disponibles)
        if (coloredSegmentsGeoJsonRef.value && map.value.getSource('colored-segments')) {
            // Aller
            if (!map.value.getLayer('trace-overlap-aller')) {
                map.value.addLayer({
                    id: 'trace-overlap-aller',
                    type: 'line',
                    source: 'colored-segments',
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                    paint: {
                        'line-width': traceWidth,
                        'line-opacity': traceOpacity,
                        'line-color': ['get', 'color_raw']
                    },
                    filter: ['!=', ['get', 'segment_type'], 'retour_overlap']
                });
            }
            // Retour
            if (!map.value.getLayer('trace-overlap-retour')) {
                map.value.addLayer({
                    id: 'trace-overlap-retour',
                    type: 'line',
                    source: 'colored-segments',
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' }, // Caché par défaut
                    paint: {
                        'line-width': traceWidth,
                        'line-opacity': traceOpacity,
                        'line-color': ['get', 'color_raw']
                    },
                    filter: ['!=', ['get', 'segment_type'], 'aller_overlap']
                });
            }
        }

        // 5. Layer Comète
        if (!map.value.getSource('comet-source')) {
            map.value.addSource('comet-source', { type: 'geojson', data: { type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} } });
        }
        if (!map.value.getLayer('comet-layer')) {
            map.value.addLayer({
                id: 'comet-layer',
                type: 'line',
                source: 'comet-source',
                paint: {
                    'line-width': cometWidth,
                    'line-color': cometColor,
                    'line-opacity': cometOpacity
                }
            });
        }
    };

    const updateLayerVisibility = (layerId, isVisible) => {
        if (!map.value || !map.value.getLayer(layerId)) return;
        map.value.setLayoutProperty(layerId, 'visibility', isVisible ? 'visible' : 'none');
    };

    const updateTraceOverlapVisibility = (zoneId, direction) => {
        if (!map.value) return;

        const showAllerOverlay = (direction === 'aller');
        const showRetourOverlay = (direction === 'retour');

        updateLayerVisibility('trace-overlap-aller', showAllerOverlay);
        updateLayerVisibility('trace-overlap-retour', showRetourOverlay);

        // Hide complete trace if overlaps are active to prevent color bleeding
        // If either aller or retour overlap is active/possible, or if we have colored segments generally?
        // Logic: if coloredSegmentsGeoJsonRef is present, we likely rely on it.
        // But strictly: if we are showing overlays, hide the base trace.
        // However, overlapping zones might not cover the whole trace?
        // Assuming coloredSegments covers everything if present.
        if (coloredSegmentsGeoJsonRef.value) {
            updateLayerVisibility('trace-complete', false);
        } else {
            updateLayerVisibility('trace-complete', true);
        }
    };

    return {
        setupTraceLayers,
        updateLayerVisibility,
        updateTraceOverlapVisibility,
        coloredSegmentsGeoJsonRef
    };
}
