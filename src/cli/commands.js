import { handleSlashCommand } from './commandHandlers/index.js';

export async function handleCommand(command, ctx) {
    return handleSlashCommand(command, ctx);
}
