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

        // Prepare coordinates
        const lats = sampledPoints.map(p => p.lat);
        const lons = sampledPoints.map(p => p.lon);

        // Open-Meteo V1 Forecast
        const baseUrl = "https://api.open-meteo.com/v1/forecast";
        const params = new URLSearchParams({
            latitude: lats.join(','),
            longitude: lons.join(','),
            hourly: "temperature_2m,apparent_temperature,precipitation_probability,precipitation,weather_code,visibility,wind_speed_10m,wind_direction_10m,wind_gusts_10m,uv_index",
            start_date: dateStr,
            end_date: dateStr, // Single day query
            timezone: "auto"
        });

        try {
            const response = await fetch(`${baseUrl}?${params.toString()} `);
            if (!response.ok) {
                throw new Error(`Weather API error: ${response.statusText} `);
            }
            let data = await response.json();

            // Normalize: If single point, data is object. If multiple, array.
            if (!Array.isArray(data)) {
                data = [data]; // Wrap in array
            }

            // Map results to our structure
            return data.map((locData, index) => {
                const point = sampledPoints[index];
                const hoursData = {};

                if (locData.hourly && locData.hourly.time) {
                    locData.hourly.time.forEach((isoTime, timeIdx) => {
                        // isoTime example: "2024-05-20T08:00"
                        // extract hour
                        const t = new Date(isoTime);
                        const h = t.getHours();

                        if (h >= startHour && h <= endHour) {
                            // Store data for this hour
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
                    hours: hoursData
                };
            });

        } catch (error) {
            console.error("Failed to fetch weather matrix:", error);
            return [];
        }
    }
};

export default WeatherService;
