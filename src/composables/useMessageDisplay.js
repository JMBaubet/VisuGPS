import { computed } from 'vue';
import { useSettings } from '@/composables/useSettings';

export function useMessageDisplay() {
    const { getSettingValue } = useSettings();

    const baseMessageFontSize = computed(() => getSettingValue('Visualisation/Messages/baseFontSize'));
    const coefLargeurMessage = computed(() => getSettingValue('Visualisation/Messages/coefLargeurMessage'));
    const coefReducteurMessage = computed(() => getSettingValue('Visualisation/Messages/coefReducteurMessage'));

    const createMessageSVG = (message) => {
        const text = message.message.text;
        const backgroundColor = message.message.style.backgroundColor || 'white';
        const textColor = message.message.style.textColor || 'black';
        const orientation = message.orientation || 'Droite';

        const designBaseFontSize = 100;
        const fontScaleFactor = baseMessageFontSize.value / designBaseFontSize;
        const fontSize = baseMessageFontSize.value;
        const baseRectHeight = 150 * fontScaleFactor;
        const rectRx = 20 * fontScaleFactor;
        const minRectWidth = 300 * fontScaleFactor;
        const padding = 50 * fontScaleFactor;

        // Formule avec réduction pour les longs messages :
        // Coef effectif = coefLargeur / (1 + (longueur * coefReducteur))
        const effectiveCoef = (coefLargeurMessage.value || 0.6) / (1 + (text.length * (coefReducteurMessage.value || 0.0)));
        const averageCharWidth = fontSize * effectiveCoef;
        const estimatedTextWidth = text.length * averageCharWidth;
        const rectWidth = Math.max(minRectWidth, estimatedTextWidth + padding);

        const skewAngle = orientation === 'Gauche' ? 20 : -20;
        const transform = `skewY(${skewAngle})`;

        const skewOffset = Math.abs(rectWidth * Math.tan(skewAngle * Math.PI / 180));
        const viewBoxHeight = baseRectHeight + skewOffset;
        const viewBoxWidth = rectWidth;

        const rectY = (orientation === 'Gauche') ? 0 : skewOffset;
        const textY = rectY + (baseRectHeight / 2) + (fontSize / 3);
        const textX = padding / 2;

        return `
            <svg width="${viewBoxWidth}" height="${viewBoxHeight}" viewBox="0 0 ${viewBoxWidth} ${viewBoxHeight}" xmlns="http://www.w3.org/2000/svg">
            <rect fill="${backgroundColor}" x="0" y="${rectY}" width="${rectWidth}" height="${baseRectHeight}" rx="${rectRx}" transform="${transform}" />
            <text text-anchor="start" x="${textX}" y="${textY}" transform="${transform}">
                ${text}
            </text>
            <style>
                <![CDATA[
                text {
                font: bold ${fontSize}px Roboto, sans-serif;
                fill: ${textColor};
                }
                ]]>
            </style>
            </svg>
        `;
    };

    return { createMessageSVG };
}
