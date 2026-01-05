import { invoke } from '@tauri-apps/api/core';

const WeatherService = {
    /**
     * Calculate timing for each point in the track based on start time and average speed.
     * @param {Array} trackPoints - Array of points [lon, lat, ele] or objects.
     * @param {String} startTimeStr - Start time in "HH:mm" format.
     * @param {Number} averageSpeedKmH - Average speed in km/h.
     * @param {String} dateStr - Date in "YYYY-MM-DD" format.
     * @returns {Array} - Track points with added 'timestamp' property (Date object).
     */
    calculateTimings(trackPoints, startTimeStr, averageSpeedKmH, dateStr) {
        if (!trackPoints || trackPoints.length === 0) return [];

        const [startHour, startMinute] = startTimeStr.split(':').map(Number);
        const startDate = new Date(dateStr);
        startDate.setHours(startHour, startMinute, 0, 0);

        const speedMS = averageSpeedKmH / 3.6;
        let currentTimestamp = startDate.getTime();
        let pointsWithTime = [];

        // First point is at start time
        pointsWithTime.push({
            ...trackPoints[0],
            timestamp: new Date(currentTimestamp),
            distance: 0
        });

        for (let i = 1; i < trackPoints.length; i++) {
            const prev = trackPoints[i - 1];
            const curr = trackPoints[i];

            // Calculate distance between prev and curr (Haversine or simple approximation if points are close)
            // Assuming trackPoints have coordinates in [lon, lat]
            const dist = this.calculateDistance(prev[1], prev[0], curr[1], curr[0]);

            const timeDiffMS = (dist / speedMS) * 1000;
            currentTimestamp += timeDiffMS;

            pointsWithTime.push({
                ...curr,
                timestamp: new Date(currentTimestamp),
                distance: pointsWithTime[i - 1].distance + dist
            });
        }

        return pointsWithTime;
    },

    /**
     * Filter points to get one approximately every intervalMeters.
     * @param {Array} trackPointsWithTime - Points with timestamp and distance.
     * @param {Number} intervalMeters - Interval in meters (default 1000).
     * @returns {Array} - Sampled points.
     */
    samplePoints(trackPointsWithTime, intervalMeters = 1000) {
        if (!trackPointsWithTime || trackPointsWithTime.length === 0) return [];

        let sampled = [trackPointsWithTime[0]];
        let lastDistance = trackPointsWithTime[0].distance;

        for (let i = 1; i < trackPointsWithTime.length; i++) {
            const point = trackPointsWithTime[i];
            if (point.distance - lastDistance >= intervalMeters) {
                sampled.push(point);
                lastDistance = point.distance;
            }
        }
        // Always include the last point if it's significantly far from the last sampled one
        const lastPoint = trackPointsWithTime[trackPointsWithTime.length - 1];
        if (lastPoint.distance - lastDistance > (intervalMeters / 2)) {
            sampled.push(lastPoint);
        } else if (sampled[sampled.length - 1] !== lastPoint) {
            // Optionally replace the last one if it's very close (not doing here for simplicity)
            // But usually we want the arrival time.
            sampled.push(lastPoint);
        }

        return sampled;
    },

    calculateDistance(lat1, lon1, lat2, lon2) {
        const R = 6371e3; // metres
        const φ1 = lat1 * Math.PI / 180; // φ, λ in radians
        const φ2 = lat2 * Math.PI / 180;
        const Δφ = (lat2 - lat1) * Math.PI / 180;
        const Δλ = (lon2 - lon1) * Math.PI / 180;

        const a = Math.sin(Δφ / 2) * Math.sin(Δφ / 2) +
            Math.cos(φ1) * Math.cos(φ2) *
            Math.sin(Δλ / 2) * Math.sin(Δλ / 2);
        const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

        return R * c;
    },

    /**
     * Fetch weather data from Open-Meteo for sampled points.
     * Since Open-Meteo accepts multiple coordinates, we can batch requests?
     * Actually Open-Meteo recommends one request per location for forecast, but they support list of locations?
     * No, the standard free API is one location per request OR comma separated lists.
     * "You can supply a list of latitude/longitude coordinates to retrieve data for multiple locations in one request."
     * Reference: https://open-meteo.com/en/docs
     * limit is usually reasonable. We have ~1 point/km. A 100km ride is 100 points.
     * We should check if we can send 100 pairs.
     * 
     * @param {Array} sampledPoints 
     * 
     * Note: The user wants forecast "J+1" (tomorrow).
     * We need to format the date correctly.
     */
    async fetchWeatherForecast(sampledPoints) {
        if (!sampledPoints || sampledPoints.length === 0) return [];

        // Prepare coordinates
        const lats = sampledPoints.map(p => p[1]);
        const lons = sampledPoints.map(p => p[0]);

        // Construct URL
        // Params:
        // latitude=...,...
        // longitude=...,...
        // hourly=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation_probability,precipitation,weather_code,surface_pressure,cloud_cover,visibility,wind_speed_10m,wind_direction_10m,wind_gusts_10m,uv_index,is_day
        // start_date=... & end_date=... (We need the specific date of the ride)

        // We assume all points are on the same day for simplicity OR we handle the date range.
        // The points have timestamps. We should find the min and max date.

        const dates = sampledPoints.map(p => new Date(p.timestamp).toISOString().split('T')[0]);
        const uniqueDates = [...new Set(dates)].sort();
        const startDate = uniqueDates[0];
        const endDate = uniqueDates[uniqueDates.length - 1];

        const baseUrl = "https://api.open-meteo.com/v1/forecast";
        const params = new URLSearchParams({
            latitude: lats.join(','),
            longitude: lons.join(','),
            hourly: "temperature_2m,apparent_temperature,precipitation_probability,precipitation,weather_code,visibility,wind_speed_10m,wind_direction_10m,wind_gusts_10m,uv_index",
            start_date: startDate,
            end_date: endDate,
            timezone: "auto"
        });

        try {
            const response = await fetch(`${baseUrl}?${params.toString()}`);
            if (!response.ok) {
                throw new Error(`Weather API error: ${response.statusText}`);
            }
            const data = await response.json();

            // Data is an array if multiple coordinates, or object if single.
            // If multiple, data is array of objects.
            const results = Array.isArray(data) ? data : [data];

            // Process results to match points with their specific hour
            return results.map((locationData, index) => {
                const point = sampledPoints[index];
                const pointTime = point.timestamp;
                // Find the hourly index corresponding to the interval [H, H+1)
                // We want 08:30 to map to 08:00, and 08:59 to 08:00.
                let closestIndex = -1;

                locationData.hourly.time.forEach((t, i) => {
                    const tDate = new Date(t);
                    const diff = pointTime - tDate; // diff in ms
                    // Check if point is within the hour starting at tDate
                    // Allow small tolerance? No, strict hour buckets usually best for "09h-10h".
                    if (diff >= 0 && diff < 60 * 60 * 1000) {
                        closestIndex = i;
                    }
                });

                if (closestIndex === -1) return null;

                const timeIndex = closestIndex;

                if (timeIndex === -1) return null;

                return {
                    point: point,
                    weather: {
                        time: locationData.hourly.time[timeIndex],
                        temperature: locationData.hourly.temperature_2m[timeIndex],
                        apparentTemperature: locationData.hourly.apparent_temperature[timeIndex],
                        precipProb: locationData.hourly.precipitation_probability[timeIndex],
                        precip: locationData.hourly.precipitation[timeIndex],
                        code: locationData.hourly.weather_code[timeIndex],
                        visibility: locationData.hourly.visibility[timeIndex],
                        windSpeed: locationData.hourly.wind_speed_10m[timeIndex],
                        windDir: locationData.hourly.wind_direction_10m[timeIndex],
                        windGusts: locationData.hourly.wind_gusts_10m[timeIndex],
                        uvIndex: locationData.hourly.uv_index[timeIndex]
                    }
                };
            }).filter(item => item !== null);

        } catch (error) {
            console.error("Failed to fetch weather:", error);
            return [];
        }
    }
};

export default WeatherService;
