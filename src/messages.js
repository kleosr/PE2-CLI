export function buildMessages({ system, user }) {
  const messages = [];
  if (system && system.trim()) messages.push({ role: 'system', content: system });
  messages.push({ role: 'user', content: user });
  return messages;
}
