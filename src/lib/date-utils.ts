/**
 * Returns the current timezone offset in minutes.
 * Note: JavaScript's getTimezoneOffset() returns (UTC - Local).
 * We want (Local - UTC), so we negate it.
 * Example: UTC-3 (Argentina) returns 180 from getTimezoneOffset().
 * We want -180.
 * @returns {number} The timezone offset in minutes.
 */
export const getTimezoneOffsetMinutes = (): number => {
  return -new Date().getTimezoneOffset();
};
