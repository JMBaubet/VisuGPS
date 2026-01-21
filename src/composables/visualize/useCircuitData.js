import { ref, shallowRef } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export function useCircuitData() {
    const circuitData = shallowRef(null);
    const lineStringRef = shallowRef(null);
    const trackingDataRef = shallowRef(null);
    const trackingPointsWithDistanceRef = shallowRef([]);
    const eventsRef = shallowRef(null);
    const segmentMetadata = shallowRef(null);

    // Pour gérer l'état initial des traces pour les variants
    const mainTraceState = ref(null);

    const loadCircuitData = async (circuitId) => {
        try {
            // Lecture parallèle des principales données du circuit
            const [fetchedLineString, fetchedTrackingData, fetchedEvents, allCircuits, metadata] = await Promise.all([
                invoke('read_line_string_file', { circuitId }),
                invoke('read_tracking_file', { circuitId }),
                invoke('get_events', { circuitId }),
                invoke('get_circuits_for_display'),
                invoke('get_segment_metadata', { circuitId }).catch(e => {
                    console.warn("Failed to load segment metadata:", e);
                    return null;
                })
            ]);

            const currentCircuit = allCircuits.find(c => c.circuitId === circuitId);
            circuitData.value = currentCircuit;
            lineStringRef.value = fetchedLineString;
            trackingDataRef.value = fetchedTrackingData;
            eventsRef.value = fetchedEvents;
            segmentMetadata.value = metadata;

            // TODO: Charger default_var.json ici plus tard pour déterminer le variant par défaut
            // const defaultVar = await invoke('read_default_var', { circuitId }).catch(() => null);

            return {
                circuit: currentCircuit,
                lineString: fetchedLineString,
                trackingData: fetchedTrackingData,
                events: fetchedEvents,
                metadata: metadata
            };

        } catch (error) {
            console.error("Error loading circuit data:", error);
            throw error;
        }
    };

    const processTrackingData = async (lineString, trackingPoints) => {
        const processedData = await invoke('process_tracking_data', {
            lineStringGeojson: lineString,
            trackingPointsJs: trackingPoints
        });
        trackingPointsWithDistanceRef.value = processedData.processedPoints;
        return processedData;
    };

    return {
        circuitData,
        lineStringRef,
        trackingDataRef,
        trackingPointsWithDistanceRef,
        eventsRef,
        segmentMetadata,
        mainTraceState,
        loadCircuitData,
        processTrackingData
    };
}
