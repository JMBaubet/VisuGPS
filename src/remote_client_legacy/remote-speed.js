function mapSliderToSpeed(sliderValue) {
    const minSpeed = g_speed_min_value;
    const maxSpeed = g_speed_max_value;
    const defaultSpeed = SLIDER_DEFAULT_SPEED;
    const halfMaxSpeed = maxSpeed / 2;

    if (sliderValue <= 20) {
        const t = sliderValue / 20;
        return minSpeed + t * (defaultSpeed - minSpeed);
    } else if (sliderValue <= 80) {
        const t = (sliderValue - 20) / 60;
        return defaultSpeed + t * (halfMaxSpeed - defaultSpeed);
    } else {
        const t = (sliderValue - 80) / 20;
        return halfMaxSpeed + t * (maxSpeed - halfMaxSpeed);
    }
}

function mapSpeedToSlider(speed) {
    const minSpeed = g_speed_min_value;
    const maxSpeed = g_speed_max_value;
    const defaultSpeed = SLIDER_DEFAULT_SPEED;
    const halfMaxSpeed = maxSpeed / 2;

    if (speed < minSpeed) return 0;
    if (speed > maxSpeed) return 100;

    if (speed <= defaultSpeed) {
        const range = defaultSpeed - minSpeed;
        if (range <= 0) return 20;
        const t = (speed - minSpeed) / range;
        return t * 20;
    } else if (speed <= halfMaxSpeed) {
        const range = halfMaxSpeed - defaultSpeed;
        if (range <= 0) return 80;
        const t = (speed - defaultSpeed) / range;
        return 20 + t * 60;
    } else {
        const range = maxSpeed - halfMaxSpeed;
        if (range <= 0) return 100;
        const t = (speed - halfMaxSpeed) / range;
        return 80 + t * 20;
    }
}