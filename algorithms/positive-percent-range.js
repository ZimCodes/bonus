/*In a range (`min` to `max`), convert an value (+/-) into a positive percentage.*/
function percentRange(value, min, max) {
	const isNegative = min < 0;
    const total = isNegative ? Math.abs(min) + Math.abs(max) : max - min;
    const numerator = isNegative ? value + Math.abs(min) : value - min;
    return (numerator / total) * 100;
}