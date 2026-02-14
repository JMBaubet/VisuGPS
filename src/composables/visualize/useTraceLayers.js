import { ref, shallowRef } from 'vue';

export function useTraceLayers(map) {
    const coloredSegmentsGeoJsonRef = shallowRef(null);
    const slopeExpressionRef = shallowRef(null);

    // Default configuration
    const config = {
        traceWidth: 4,
        traceOpacity: 1,
        traceColor: '#FF9800', // Default Variant Track Color (if slope disabled)
        cometWidth: 5,
        cometColor: '#FF0000',
        cometOpacity: 0.8
    };

    const setupTraceLayers = (configs = {}) => {
        if (!map.value) return;

        let {
            traceWidth, traceOpacity, traceColor,
            lineStringData, // Standard Trace Source
            coloredSegmentsData, // Variant Source (Priority)
            masterTraceData, // Master Trace for Variant Main Layer
            cometWidth, cometColor, cometOpacity,
            // Variant specific
            segmentThickness, segmentOpacity,
            colorNew, colorCommon, colorAbandoned,
            // Slope specific
            slopeThickness, slopeOpacityLogic, slopeExpression
        } = { ...config, ...configs };

        // AGGRESSIVE SANITIZATION: Strip problematic properties that trigger Mapbox v3 warnings
        const sanitize = (data) => {
            if (!data) return;
            const features = data.features || (data.type === 'Feature' ? [data] : []);
            features.forEach(f => {
                if (f.properties) {
                    const { icon, background, 'background-stroke': bgS, ...safe } = f.properties;
                    f.properties = safe;
                }
            });
        };
        sanitize(lineStringData);
        sanitize(coloredSegmentsData);
        sanitize(masterTraceData);

        if (slopeExpression) slopeExpressionRef.value = slopeExpression;
        if (coloredSegmentsData) coloredSegmentsGeoJsonRef.value = coloredSegmentsData;

        // --- MODE 1: VARIANT (3-Layer Architecture) ---
        if (coloredSegmentsGeoJsonRef.value) {

            // Source: Colored Segments
            if (!map.value.getSource('colored-segments')) {
                map.value.addSource('colored-segments', { type: 'geojson', data: coloredSegmentsGeoJsonRef.value, lineMetrics: true });
            } else {
                // Force update data if source exists (critical for switching variants)
                map.value.getSource('colored-segments').setData(coloredSegmentsGeoJsonRef.value);
            }

            // Source: Master Trace (for Main/Abandoned layer)
            if (masterTraceData) {
                if (!map.value.getSource('trace-master-source')) {
                    map.value.addSource('trace-master-source', { type: 'geojson', data: masterTraceData, lineMetrics: true });
                } else {
                    map.value.getSource('trace-master-source').setData(masterTraceData);
                }
            }

            // Layer 1: Main / Abandoned (using Master Trace)
            // Use masterTraceData if available, otherwise fallback (though masterTraceData is expected in Variant mode)
            const mainSourceId = masterTraceData ? 'trace-master-source' : 'colored-segments';
            const mainFilter = masterTraceData ? undefined : ['==', ['get', 'status'], 'ABANDONED']; // No filter if using full master trace



            if (!map.value.getLayer('trace-main-abandoned')) {

                const layerOptions = {
                    id: 'trace-main-abandoned',
                    type: 'line',
                    source: mainSourceId,
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                    paint: {
                        'line-width': Math.max(1, (segmentThickness || traceWidth) - 1),
                        'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity,
                        'line-color': colorAbandoned || '#000000'
                    }
                };

                if (mainFilter) {
                    layerOptions.filter = mainFilter;
                }

                map.value.addLayer(layerOptions);
            } else {
                // Refresh existing layer

                const currentSource = map.value.getLayer('trace-main-abandoned').source;

                // If source changed (e.g. from colored-segments to master-source), we MUST recreate the layer.
                if (currentSource !== mainSourceId) {

                    map.value.removeLayer('trace-main-abandoned');
                    const layerOptions = {
                        id: 'trace-main-abandoned',
                        type: 'line',
                        source: mainSourceId,
                        layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                        paint: {
                            'line-width': Math.max(1, (segmentThickness || traceWidth) - 1),
                            'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity,
                            'line-color': colorAbandoned || '#000000'
                        }
                    };
                    if (mainFilter) layerOptions.filter = mainFilter;
                    map.value.addLayer(layerOptions);
                } else {
                    // Just update filter and paint
                    if (mainFilter) {
                        map.value.setFilter('trace-main-abandoned', mainFilter);
                    } else {
                        map.value.setFilter('trace-main-abandoned', null);
                    }
                    map.value.setPaintProperty('trace-main-abandoned', 'line-color', colorAbandoned || '#000000');
                    map.value.setPaintProperty('trace-main-abandoned', 'line-width', Math.max(1, (segmentThickness || traceWidth) - 1));
                }
            }

            // Layer 2: Segments (Common)
            const layerCommonId = 'trace-variant-segment-common';
            if (!map.value.getLayer(layerCommonId)) {
                map.value.addLayer({
                    id: layerCommonId,
                    type: 'line',
                    source: 'colored-segments',
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                    paint: {
                        'line-width': segmentThickness || traceWidth,
                        'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity,
                        'line-color': colorCommon || '#4CAF50'
                    },
                    filter: ['==', ['get', 'status'], 'COMMON']
                });
            }

            // Layer 2: Segments (New)
            const layerNewId = 'trace-variant-segment-new';
            if (!map.value.getLayer(layerNewId)) {
                map.value.addLayer({
                    id: layerNewId,
                    type: 'line',
                    source: 'colored-segments',
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                    paint: {
                        'line-width': segmentThickness || traceWidth,
                        'line-opacity': segmentOpacity !== undefined ? segmentOpacity : traceOpacity,
                        'line-color': colorNew || '#2196F3'
                    },
                    filter: ['==', ['get', 'status'], 'NEW']
                });
            }

            // Layer 3: Slope / Full Trace (Aller)
            if (!map.value.getLayer('trace-slope-aller')) {
                map.value.addLayer({
                    id: 'trace-slope-aller',
                    type: 'line',
                    source: 'colored-segments',
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                    paint: {
                        'line-width': slopeThickness || traceWidth,
                        'line-opacity': slopeOpacityLogic !== undefined ? slopeOpacityLogic : traceOpacity,
                        'line-color': ['get', 'color_raw']
                    },
                    filter: ['all', ['!=', ['get', 'segment_type'], 'retour_overlap'], ['!=', ['get', 'status'], 'ABANDONED']]
                });
            }

            // Layer 3: Slope / Full Trace (Retour)
            if (!map.value.getLayer('trace-slope-retour')) {
                map.value.addLayer({
                    id: 'trace-slope-retour',
                    type: 'line',
                    source: 'colored-segments',
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' }, // Default hidden
                    paint: {
                        'line-width': slopeThickness || traceWidth,
                        'line-opacity': slopeOpacityLogic !== undefined ? slopeOpacityLogic : traceOpacity,
                        'line-color': ['get', 'color_raw']
                    },
                    filter: ['all', ['!=', ['get', 'segment_type'], 'aller_overlap'], ['!=', ['get', 'status'], 'ABANDONED']]
                });
            }

            // --- ENFORCE Z-INDEX ORDER (Bottom to Top) ---
            const variantLayerOrder = [
                'trace-main-abandoned',
                'trace-variant-segment-common',
                'trace-variant-segment-new',
                'trace-slope-aller',
                'trace-slope-retour',
                'comet-layer'
            ];

            // Re-order layers by moving them to the top in the desired sequence
            variantLayerOrder.forEach(layerId => {
                if (map.value.getLayer(layerId)) {
                    map.value.moveLayer(layerId);
                }
            });

            // Hide legacy standard layers if they exist
            updateLayerVisibility('trace-complete', false);
            updateLayerVisibility('trace-slope', false);

        }
        // --- MODE 2: STANDARD TRACE (Fallback) ---
        else if (lineStringData) {

            // Source: Standard Trace
            if (!map.value.getSource('trace')) {
                map.value.addSource('trace', { type: 'geojson', data: lineStringData, lineMetrics: true });
            }

            // Layer: Simple / Gradient (Unified naming 'trace-slope-aller' for compatibility)
            // Note: Main trace usually implies 'aller' direction only unless overlap logic is added later.
            // We use 'trace-slope-aller' as the primary visible layer.
            const useSlope = !!slopeExpressionRef.value;
            const layerId = 'trace-slope-aller';

            if (!map.value.getLayer(layerId)) {
                map.value.addLayer({
                    id: layerId,
                    type: 'line',
                    source: 'trace',
                    layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                    paint: {
                        'line-width': traceWidth,
                        'line-opacity': traceOpacity,
                        'line-gradient': useSlope ? slopeExpressionRef.value : undefined,
                        'line-color': !useSlope ? traceColor : '#000000'
                    }
                });
            }

            // Ensure no legacy layers remain
            if (map.value.getLayer('trace-slope')) map.value.removeLayer('trace-slope');
            if (map.value.getLayer('trace-complete')) map.value.removeLayer('trace-complete');
        }

        // 4. Layer Comète (Shared)
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

    /**
     * Controls visibility of 'Main' and 'Segment' layers.
     * 'Slope' layer is always visible (but its style changes).
     */
    const updateLayerVisibility = (layerTypeIsSegments, isVisible) => {
        if (!map.value) return;

        // Variant Layers
        // Variant Segments (Common/New)
        // Note: 'Main/Abandoned' is now controlled separately via specific call
        if (typeof layerTypeIsSegments === 'boolean') {
            ['trace-variant-segment-common', 'trace-variant-segment-new'].forEach(id => {
                if (map.value.getLayer(id)) {
                    map.value.setLayoutProperty(id, 'visibility', isVisible ? 'visible' : 'none');
                }
            });
        }

        // Standard Layers / Specific Layer Toggle
        if (typeof layerTypeIsSegments === 'string') {
            const id = layerTypeIsSegments;

            if (map.value.getLayer(id)) {
                map.value.setLayoutProperty(id, 'visibility', isVisible ? 'visible' : 'none');
            }
        }
    };

    /**
     * Handles Aller/Retour switching for the 'Slope' layer ONLY (Variant Mode).
     */
    const updateTraceOverlapVisibility = (zoneId, direction) => {
        if (!map.value) return;
        // Only applies to Variant Mode layers
        const showAller = (direction === 'aller');
        const showRetour = (direction === 'retour');

        if (map.value.getLayer('trace-slope-aller')) {
            map.value.setLayoutProperty('trace-slope-aller', 'visibility', showAller ? 'visible' : 'none');
        }
        if (map.value.getLayer('trace-slope-retour')) {
            map.value.setLayoutProperty('trace-slope-retour', 'visibility', showRetour ? 'visible' : 'none');
        }
    };

    /**
     * Switches Slope Layer style between Gradient (Pente) and Flat Color.
     */
    const updateVariantSlopeMode = (showSlopeGradient, colors = {}) => {
        if (!map.value) return;

        // Variant Mode Layers & Unified Main Trace Layer
        const flatColor = colors.trace || '#FF9800';
        ['trace-slope-aller', 'trace-slope-retour'].forEach(layerId => {
            if (map.value.getLayer(layerId)) {
                if (showSlopeGradient) {
                    if (map.value.getSource('colored-segments')) {
                        // Variant Mode: Use 'color_raw' property from features
                        map.value.setPaintProperty(layerId, 'line-color', ['get', 'color_raw']);
                    } else {
                        // Main Trace Mode: Use Gradient Expression if available, else flat
                        // The gradient is set via 'line-gradient' property on the layer, not 'line-color'.
                        // 'line-color' is ignored if 'line-gradient' is set ?? No, Mapbox Spec:
                        // line-gradient requires source type 'geojson' with 'lineMetrics: true'.
                        // If line-gradient is set, line-color is typically ignored or used as fallback?
                        // Actually, to switch Main Trace to FLAT color, we must unset line-gradient?
                        // STARTUP LOGIC SETS 'line-gradient'. 
                        // To toggle LIVE:
                        if (slopeExpressionRef.value) {
                            map.value.setPaintProperty(layerId, 'line-gradient', slopeExpressionRef.value); // Restore
                            map.value.setPaintProperty(layerId, 'line-color', '#000000'); // Dummy
                        }
                    }
                } else {
                    // Mode Trace (Flat)
                    // For Main Trace, we must remove/disable line-gradient first
                    map.value.setPaintProperty(layerId, 'line-gradient', null);
                    map.value.setPaintProperty(layerId, 'line-color', flatColor);
                }
            }
        });
    };

    /**
     * Updates Styles (Thickness, Opacity, Colors) for all layers live.
     */
    const updateVariantStyle = (styles = {}) => {
        if (!map.value) return;
        const { thickness, opacity, colorNew, colorCommon, colorAbandoned, colorTrace,
            slopeThickness, slopeOpacity } = styles;

        // 1. Update Main (Abandoned)
        if (map.value.getLayer('trace-main-abandoned')) {
            if (thickness !== undefined) map.value.setPaintProperty('trace-main-abandoned', 'line-width', Math.max(1, thickness - 1));
            if (opacity !== undefined) map.value.setPaintProperty('trace-main-abandoned', 'line-opacity', opacity);
            if (colorAbandoned !== undefined) map.value.setPaintProperty('trace-main-abandoned', 'line-color', colorAbandoned);
        }

        // 2. Update Segments (Common/New)
        if (map.value.getLayer('trace-variant-segment-common')) {
            if (thickness !== undefined) map.value.setPaintProperty('trace-variant-segment-common', 'line-width', thickness);
            if (opacity !== undefined) map.value.setPaintProperty('trace-variant-segment-common', 'line-opacity', opacity);
            if (colorCommon !== undefined) map.value.setPaintProperty('trace-variant-segment-common', 'line-color', colorCommon);
        }
        if (map.value.getLayer('trace-variant-segment-new')) {
            if (thickness !== undefined) map.value.setPaintProperty('trace-variant-segment-new', 'line-width', thickness);
            if (opacity !== undefined) map.value.setPaintProperty('trace-variant-segment-new', 'line-opacity', opacity);
            if (colorNew !== undefined) map.value.setPaintProperty('trace-variant-segment-new', 'line-color', colorNew);
        }

        // 3. Update Slope (Top)
        ['trace-slope-aller', 'trace-slope-retour'].forEach(id => {
            if (map.value.getLayer(id)) {
                if (slopeThickness !== undefined) map.value.setPaintProperty(id, 'line-width', slopeThickness);
                if (slopeOpacity !== undefined) map.value.setPaintProperty(id, 'line-opacity', slopeOpacity);

                const currentGradient = map.value.getPaintProperty(id, 'line-gradient');
                if (!currentGradient && colorTrace !== undefined) {
                    map.value.setPaintProperty(id, 'line-color', colorTrace);
                }
            }
        });
    };

    return {
        setupTraceLayers,
        updateLayerVisibility,
        updateTraceOverlapVisibility,
        updateVariantSlopeMode,
        updateVariantStyle,
        coloredSegmentsGeoJsonRef,
        slopeExpressionRef
    };
}
