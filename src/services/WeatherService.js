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
    }
};

export default WeatherService;
