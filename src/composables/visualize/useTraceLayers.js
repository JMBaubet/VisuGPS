import { ref, shallowRef } from 'vue';

export function useTraceLayers(map) {
    const coloredSegmentsGeoJsonRef = shallowRef(null);
    const slopeExpressionRef = shallowRef(null);

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
            cometWidth, cometColor, cometOpacity,
            // Variant specific info
            segmentThickness, segmentOpacity,
            colorNew, colorCommon, colorAbandoned,
            // Slope specific info
            slopeThickness, slopeOpacityLogic, slopeExpression
        } = { ...config, ...configs };

        if (slopeExpression) {
            slopeExpressionRef.value = slopeExpression;
        }

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

        // 6. Layers Variantes
        if (coloredSegmentsGeoJsonRef.value) {
            const hasStatus = coloredSegmentsGeoJsonRef.value.features?.some(f => f.properties && f.properties.status);
            if (hasStatus) {
                // Background: Abandoned
                if (!map.value.getLayer('trace-variant-abandoned')) {
                    map.value.addLayer({
                        id: 'trace-variant-abandoned',
                        type: 'line',
                        source: 'colored-segments',
                        layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' },
                        paint: { 'line-width': segmentThickness || traceWidth, 'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity, 'line-color': colorAbandoned || '#000000' },
                        filter: ['==', ['get', 'status'], 'ABANDONED']
                    });
                }

                // Common segments
                if (!map.value.getLayer('trace-variant-common-aller')) {
                    map.value.addLayer({
                        id: 'trace-variant-common-aller',
                        type: 'line',
                        source: 'colored-segments',
                        layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                        paint: { 'line-width': segmentThickness || traceWidth, 'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity, 'line-color': colorCommon || '#4CAF50' },
                        filter: ['all', ['==', ['get', 'status'], 'COMMON'], ['!=', ['get', 'segment_type'], 'retour_overlap']]
                    });
                }
                if (!map.value.getLayer('trace-variant-common-retour')) {
                    map.value.addLayer({
                        id: 'trace-variant-common-retour',
                        type: 'line',
                        source: 'colored-segments',
                        layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' },
                        paint: { 'line-width': segmentThickness || traceWidth, 'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity, 'line-color': colorCommon || '#4CAF50' },
                        filter: ['all', ['==', ['get', 'status'], 'COMMON'], ['!=', ['get', 'segment_type'], 'aller_overlap']]
                    });
                }

                // New segments
                if (!map.value.getLayer('trace-variant-new-aller')) {
                    map.value.addLayer({
                        id: 'trace-variant-new-aller',
                        type: 'line',
                        source: 'colored-segments',
                        layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                        paint: { 'line-width': segmentThickness || traceWidth, 'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity, 'line-color': colorNew || '#2196F3' },
                        filter: ['all', ['==', ['get', 'status'], 'NEW'], ['!=', ['get', 'segment_type'], 'retour_overlap']]
                    });
                }
                if (!map.value.getLayer('trace-variant-new-retour')) {
                    map.value.addLayer({
                        id: 'trace-variant-new-retour',
                        type: 'line',
                        source: 'colored-segments',
                        layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' },
                        paint: { 'line-width': segmentThickness || traceWidth, 'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity, 'line-color': colorNew || '#2196F3' },
                        filter: ['all', ['==', ['get', 'status'], 'NEW'], ['!=', ['get', 'segment_type'], 'aller_overlap']]
                    });
                }

                updateLayerVisibility('trace-complete', false);
                updateLayerVisibility('trace-overlap-aller', false);
                updateLayerVisibility('trace-overlap-retour', false);
            }
        }

        // 7. Layer Pente (Slope) - For Main Trace
        if (slopeExpressionRef.value && !map.value.getLayer('trace-slope')) {
            map.value.addLayer({
                id: 'trace-slope',
                type: 'line',
                source: 'trace',
                layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' },
                paint: {
                    'line-width': slopeThickness || traceWidth,
                    'line-opacity': slopeOpacityLogic !== undefined ? slopeOpacityLogic : traceOpacity,
                    'line-gradient': slopeExpressionRef.value
                }
            });
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

    const updateTraceOverlapVisibility = (zoneId, direction, forceVisible = true) => {
        if (!map.value) return;

        const showAllerOverlay = forceVisible && (direction === 'aller');
        const showRetourOverlay = forceVisible && (direction === 'retour');

        updateLayerVisibility('trace-overlap-aller', showAllerOverlay);
        updateLayerVisibility('trace-overlap-retour', showRetourOverlay);

        updateLayerVisibility('trace-variant-common-aller', showAllerOverlay);
        updateLayerVisibility('trace-variant-common-retour', showRetourOverlay);
        updateLayerVisibility('trace-variant-new-aller', showAllerOverlay);
        updateLayerVisibility('trace-variant-new-retour', showRetourOverlay);

        if (coloredSegmentsGeoJsonRef.value) {
            updateLayerVisibility('trace-complete', false);
        } else {
            updateLayerVisibility('trace-complete', forceVisible);
        }
    };

    const updateVariantSlopeMode = (enabled, colors = {}) => {
        if (!map.value) return;
        const layers = ['trace-variant-common-aller', 'trace-variant-common-retour', 'trace-variant-new-aller', 'trace-variant-new-retour'];
        layers.forEach(layerId => {
            if (map.value.getLayer(layerId)) {
                let baseColor = colors.common || '#4CAF50';
                if (layerId.includes('-new-')) baseColor = colors.new || '#2196F3';
                map.value.setPaintProperty(layerId, 'line-color', enabled ? ['get', 'color_raw'] : baseColor);
            }
        });
        // Also toggle the main trace-slope if it exists (for loop parts)
        updateLayerVisibility('trace-slope', enabled && !coloredSegmentsGeoJsonRef.value);
    };

    return {
        setupTraceLayers,
        updateLayerVisibility,
        updateTraceOverlapVisibility,
        updateVariantSlopeMode,
        coloredSegmentsGeoJsonRef,
        slopeExpressionRef
    };
}
