
const setupRemoteControl = async () => {
    console.log("Setting up remote control listeners...");
    // Clear existing if any (safety)
    // unlistenFunctions.forEach(fn => fn()); // Don't do this here if it clears map listeners too!
    // Assuming unlistenFunctions stores generic cleanup. 
    // Let's create a dedicated array for remote listeners to avoid clearing important map stuff?
    // For now, just add.

    const unlistenTogglePlay = await listen('remote_command::toggle_play', () => {
        console.log("Remote: Toggle Play Received");
        togglePlayPauseOrReset();
    });
    unlistenFunctions.push(unlistenTogglePlay);

    const unlistenRestart = await listen('remote_command::restart_animation', () => {
        resetAnimation();
    });
    unlistenFunctions.push(unlistenRestart);

    const unlistenRewindStart = await listen('remote_command::start_rewind', () => {
        isRewinding.value = true;
    });
    unlistenFunctions.push(unlistenRewindStart);

    const unlistenRewindStop = await listen('remote_command::stop_rewind', () => {
        isRewinding.value = false;
    });
    unlistenFunctions.push(unlistenRewindStop);

    const unlistenUpdateSpeed = await listen('remote_command::update_speed', (event) => {
        if (event.payload && typeof event.payload.speed === 'number') {
            currentSpeed.value = event.payload.speed;
        }
    });
    unlistenFunctions.push(unlistenUpdateSpeed);

    const unlistenToggleCommands = await listen('remote_command::toggle_commands_widget', () => {
        isControlsCardVisible.value = !isControlsCardVisible.value;
    });
    unlistenFunctions.push(unlistenToggleCommands);

    const unlistenToggleProfile = await listen('remote_command::toggle_altitude_profile', () => {
        isAltitudeVisible.value = !isAltitudeVisible.value;
    });
    unlistenFunctions.push(unlistenToggleProfile);

    const unlistenToggleCommunes = await listen('remote_command::toggle_communes_display', () => {
        isCommuneWidgetVisible.value = !isCommuneWidgetVisible.value;
    });
    unlistenFunctions.push(unlistenToggleCommunes);

    const unlistenToggleDistance = await listen('remote_command::toggle_distance_display', () => {
        isDistanceDisplayVisible.value = !isDistanceDisplayVisible.value;
    });
    unlistenFunctions.push(unlistenToggleDistance);

    const unlistenToggleWeatherDyn = await listen('remote_command::toggle_weather_dynamic', () => {
        isDynamicWeatherVisible.value = !isDynamicWeatherVisible.value;
        isCompassVisible.value = isDynamicWeatherVisible.value;
    });
    unlistenFunctions.push(unlistenToggleWeatherDyn);

    const unlistenToggleWeatherStatic = await listen('remote_command::toggle_weather_static', () => {
        isStaticWeatherVisible.value = !isStaticWeatherVisible.value;
        showWeatherTable.value = !showWeatherTable.value;
    });
    unlistenFunctions.push(unlistenToggleWeatherStatic);
};
