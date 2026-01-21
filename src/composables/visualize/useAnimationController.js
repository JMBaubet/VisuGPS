import { ref, computed } from 'vue';
import * as turf from '@turf/turf';

export function useAnimationController() {
    const isPaused = ref(true);
    const isRewinding = ref(false);
    const isAnimationFinished = ref(false);
    const isInitializing = ref(true); // Géré par la vue pour le moment

    // État temporel
    let accumulatedTime = 0;
    let lastTimestamp = 0;
    let animationFrameId = null;

    // Vitesse
    const currentSpeed = ref(1.0); // Sera synchronisé par la vue

    // Données de progression (reactive pour la vue)
    const currentDistanceInMeters = ref(0);
    const distanceDisplay = ref('0.00');

    // Pour l'interpolation
    const currentTraceBearing = ref(0);

    const startAnimation = (animateCallback) => {
        isPaused.value = false;
        isAnimationFinished.value = false;
        lastTimestamp = 0;
        animationFrameId = requestAnimationFrame(animateCallback);
    };

    const pauseAnimation = () => {
        isPaused.value = true;
        if (animationFrameId) {
            cancelAnimationFrame(animationFrameId);
            animationFrameId = null;
        }
    };

    const resetTime = () => {
        accumulatedTime = 0;
        lastTimestamp = 0;
        currentDistanceInMeters.value = 0;
    };

    const updateTime = (deltaTime, totalDuration, totalDistance) => {
        if (isRewinding.value) {
            accumulatedTime = Math.max(0, accumulatedTime - (deltaTime * 2 * currentSpeed.value));
        } else {
            accumulatedTime += deltaTime * currentSpeed.value;
        }

        const phase = totalDuration > 0 ? Math.min(accumulatedTime / totalDuration, 1) : 1;
        const distanceTraveled = totalDistance * phase;

        currentDistanceInMeters.value = distanceTraveled * 1000;
        distanceDisplay.value = distanceTraveled.toFixed(2);

        return { phase, distanceTraveled };
    };

    // Helper Math
    const lerp = (start, end, amt) => (1 - amt) * start + amt * end;
    const lerpAngle = (start, end, amt) => {
        const da = (end - start) % 360;
        const shortestAngle = (2 * da % 360) - da;
        return start + shortestAngle * amt;
    };

    return {
        isPaused,
        isRewinding,
        isAnimationFinished,
        currentSpeed,
        currentDistanceInMeters,
        distanceDisplay,
        currentTraceBearing,
        accumulatedTime, // Note: Exposed as let, might need wrapping if modified directly by view
        startAnimation,
        pauseAnimation,
        resetTime,
        updateTime,
        lerp,
        lerpAngle
    };
}
