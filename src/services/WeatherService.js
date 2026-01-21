const WeatherService = {

    /**
     * Fetch weather data from Open-Meteo for a specific day and hour range.
     * @param {Array} sampledPoints - Array of { lat, lon, increment }
     * @param {String} dateStr - "YYYY-MM-DD"
     * @param {Number} startHour - e.g. 6
     * @param {Number} endHour - e.g. 20
     * @returns {Array} - Array of { increment, hours: { [hour]: { temp... } } }
     */
    async fetchWeatherMatrix(sampledPoints, dateStr, startHour = 6, endHour = 20) {
        if (!sampledPoints || sampledPoints.length === 0) return [];

        const CHUNK_SIZE = 100;
        const chunks = [];

        for (let i = 0; i < sampledPoints.length; i += CHUNK_SIZE) {
            chunks.push(sampledPoints.slice(i, i + CHUNK_SIZE));
        }

        const baseUrl = "https://api.open-meteo.com/v1/forecast";

        try {
            // Helper for retry logic
            const fetchWithRetry = async (url, retries = 3, backoff = 2000) => {
                for (let i = 0; i < retries; i++) {
                    const response = await fetch(url);
                    if (response.status === 429) {
                        console.warn(`Rate limit hit (429), waiting ${backoff}ms...`);
                        await new Promise(r => setTimeout(r, backoff));
                        backoff *= 2; // Exponential backoff
                        continue;
                    }
                    if (!response.ok) {
                        throw new Error(`Weather API error: ${response.statusText}`);
                    }
                    return response;
                }
                throw new Error("Max retries reached for Weather API");
            };

            const results = [];
            for (const chunk of chunks) {
                const lats = chunk.map(p => p.lat);
                const lons = chunk.map(p => p.lon);

                const params = new URLSearchParams({
                    latitude: lats.join(','),
                    longitude: lons.join(','),
                    hourly: "temperature_2m,apparent_temperature,precipitation_probability,precipitation,weather_code,visibility,wind_speed_10m,wind_direction_10m,wind_gusts_10m,uv_index",
                    start_date: dateStr,
                    end_date: dateStr, // Single day query
                    timezone: "auto"
                });

                // Use retry wrapper
                const response = await fetchWithRetry(`${baseUrl}?${params.toString()}`);

                let data = await response.json();

                // Normalize
                if (!Array.isArray(data)) {
                    data = [data];
                }

                // Map chunk results
                const chunkResult = data.map((locData, index) => {
                    const point = chunk[index];
                    const hoursData = {};

                    if (locData.hourly && locData.hourly.time) {
                        locData.hourly.time.forEach((isoTime, timeIdx) => {
                            const t = new Date(isoTime);
                            const h = t.getHours();

                            if (h >= startHour && h <= endHour) {
                                hoursData[h] = {
                                    temperature: locData.hourly.temperature_2m[timeIdx],
                                    apparentTemperature: locData.hourly.apparent_temperature[timeIdx],
                                    precipProb: locData.hourly.precipitation_probability[timeIdx],
                                    precip: locData.hourly.precipitation[timeIdx],
                                    code: locData.hourly.weather_code[timeIdx],
                                    visibility: locData.hourly.visibility[timeIdx],
                                    windSpeed: locData.hourly.wind_speed_10m[timeIdx],
                                    windDir: locData.hourly.wind_direction_10m[timeIdx],
                                    windGusts: locData.hourly.wind_gusts_10m[timeIdx],
                                    uvIndex: locData.hourly.uv_index[timeIdx],
                                    time: isoTime
                                };
                            }
                        });
                    }

                    return {
                        increment: point.increment,
                        km: point.km,
                        hours: hoursData
                    };
                });

                results.push(...chunkResult);

                // Add delay between requests (1s safe zone)
                await new Promise(resolve => setTimeout(resolve, 1000));
            }

            return results;

        } catch (error) {
            console.error("Failed to fetch weather matrix:", error);
            throw error;
        }
    },

    async generateWeatherForecasts(circuit, trackingPoints, scenarios, circuitId) {
        if (!circuit || !trackingPoints || trackingPoints.length === 0) {
            console.warn("WeatherService: Missing inputs");
            return [];
        }

        // 1. Determine Date
        let dateStr = new Date().toISOString().split('T')[0];
        if (circuit.dateDepart) {
            dateStr = new Date(circuit.dateDepart).toISOString().split('T')[0];
        }

        // Check Cache
        const cacheFilename = `${dateStr.replace(/-/g, '')}-06-to-20.json`;

        if (circuitId) {
            try {
                const { invoke } = await import('@tauri-apps/api/core');
                const cachedData = await invoke('check_weather_cache', {
                    circuitId,
                    filename: cacheFilename
                });

                if (cachedData) {
                    return JSON.parse(cachedData);
                }
            } catch (e) {
                console.warn("WeatherService: Cache check failed", e);
            }
        }

        // 3. Sample Points (every ~5 km to avoid API limits)
        const SAMPLING_DISTANCE_KM = 5;
        const sampledPoints = [];
        let lastSampledKm = -SAMPLING_DISTANCE_KM;

        trackingPoints.forEach((pt, index) => {
            if (pt.distance !== undefined) {
                if (pt.distance - lastSampledKm >= SAMPLING_DISTANCE_KM) {
                    sampledPoints.push({
                        lat: pt.lat || pt.coordonnee[1],
                        lon: pt.lon || pt.coordonnee[0],
                        increment: index,
                        km: pt.distance
                    });
                    lastSampledKm = pt.distance;
                }
            } else if (index === 0) {
                console.warn("WeatherService: Point 0 has no distance!", pt);
            }
        });

        console.log(`WeatherService: Sampled ${sampledPoints.length} points from ${trackingPoints.length} inputs.`);

        // Always include start if empty
        if (sampledPoints.length === 0 && trackingPoints.length > 0) {
            const pt = trackingPoints[0];
            sampledPoints.push({ lat: pt.lat || pt.coordonnee[1], lon: pt.lon || pt.coordonnee[0], increment: 0, km: 0 });
        }

        // 4. Fetch from API
        console.log("WeatherService: Calling fetchWeatherMatrix with", sampledPoints.length, "points");
        const result = await this.fetchWeatherMatrix(sampledPoints, dateStr);
        console.log("WeatherService: fetchWeatherMatrix returned", result?.length, "items");

        // 5. Save to Cache
        if (circuitId && result && result.length > 0) {
            try {
                const { invoke } = await import('@tauri-apps/api/core');
                await invoke('save_weather_cache', {
                    circuitId,
                    filename: cacheFilename,
                    content: JSON.stringify(result)
                });
                console.log("WeatherService: Saved to cache");
            } catch (e) {
                console.warn("WeatherService: Cache save failed", e);
            }
        }

        return result;
    },

    getCurrentWeather(currentKm, currentDate, weatherMatrix) {
        if (!weatherMatrix || weatherMatrix.length === 0 || !currentDate) return null;

        // 1. Find Closest Location
        // Sort by distance difference
        let closest = weatherMatrix[0];
        let minDiff = Math.abs(currentKm - closest.km);

        for (let i = 1; i < weatherMatrix.length; i++) {
            const diff = Math.abs(currentKm - weatherMatrix[i].km);
            if (diff < minDiff) {
                minDiff = diff;
                closest = weatherMatrix[i];
            }
        }

        // 2. Get Data for Current Hour
        const hour = currentDate.getHours();
        if (closest.hours && closest.hours[hour]) {
            return closest.hours[hour];
        }

        return null;
    }
};

export default WeatherService;
