export function assertNonEmptyString(value, message) {
  if (!value?.trim()) {
    throw new Error(message);
  }
}
