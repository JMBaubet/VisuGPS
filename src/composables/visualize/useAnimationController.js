import { ref, computed } from 'vue';
import * as turf from '@turf/turf';

export function useAnimationController() {
    const isPaused = ref(true);
    const isRewinding = ref(false);
    const isAnimationFinished = ref(false);
    const isInitializing = ref(true); // Géré par la vue pour le moment

    // État temporel
    const accumulatedTime = ref(0);
    const lastTimestamp = ref(0);
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
        lastTimestamp.value = 0;
        if (animationFrameId) cancelAnimationFrame(animationFrameId);
        animationFrameId = requestAnimationFrame(animateCallback);
    };

    const pauseAnimation = () => {
        isPaused.value = true;
        if (animationFrameId) {
            cancelAnimationFrame(animationFrameId);
            animationFrameId = null;
        }
    };

    const stopAnimation = () => {
        if (animationFrameId) {
            cancelAnimationFrame(animationFrameId);
            animationFrameId = null;
        }
    };

    const continueAnimation = (animateCallback) => {
        animationFrameId = requestAnimationFrame(animateCallback);
    };


    const resetTime = () => {
        accumulatedTime.value = 0;
        lastTimestamp.value = 0;
        currentDistanceInMeters.value = 0;
    };

    const updateTime = (deltaTime, totalDuration, totalDistance) => {
        if (isRewinding.value) {
            accumulatedTime.value = Math.max(0, accumulatedTime.value - (deltaTime * 2 * currentSpeed.value));
        } else {
            accumulatedTime.value += deltaTime * currentSpeed.value;
        }

        const phase = totalDuration > 0 ? Math.min(accumulatedTime.value / totalDuration, 1) : 1;
        const distanceTraveled = totalDistance * phase;

        currentDistanceInMeters.value = distanceTraveled * 1000;
        distanceDisplay.value = distanceTraveled.toFixed(2);

        return { phase, distanceTraveled };
    };

    /**
     * Force l'état de l'animation à une distance donnée.
     * Recalcule accumulatedTime pour correspondre à cette distance.
     */
    const setTimeFromDistance = (targetDistanceInKm, totalDistanceKm, totalDurationMs) => {
        // Protection division par zéro
        if (totalDistanceKm <= 0) return;

        // Clamp distance
        const clampedDist = Math.max(0, Math.min(targetDistanceInKm, totalDistanceKm));

        // Calcul du ratio
        const ratio = clampedDist / totalDistanceKm;

        // Mise à jour du temps interne
        accumulatedTime.value = ratio * totalDurationMs;

        // Mise à jour immédiate des refs exposées
        currentDistanceInMeters.value = clampedDist * 1000;
        distanceDisplay.value = clampedDist.toFixed(2);

        return { phase: ratio, distanceTraveled: clampedDist };
    };

    const setSpeed = (newSpeed) => {
        currentSpeed.value = newSpeed;
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
        accumulatedTime, 
        lastTimestamp,
        startAnimation,
        pauseAnimation,
        stopAnimation,
        continueAnimation,
        resetTime,
        updateTime,
        setTimeFromDistance,
        setSpeed,
        lerp,
        lerpAngle
    };
}
