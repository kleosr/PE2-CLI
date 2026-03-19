import {
    handleSettings,
    handleConfig,
    handleModel,
    handleShowkey
} from './configCommands.js';
import {
    handleHistory,
    handleExport,
    handleImport,
    handleBatch,
    handleCopy,
    handleClearall
} from './sessionCommands.js';
import {
    handleClear,
    handleTheme,
    handlePreferences,
    handleCompact
} from './preferencesCommands.js';
import { handleHelp } from './helpCommand.js';

const COMMAND_REGISTRY = new Map([
    ['/settings', handleSettings],
    ['/config', handleConfig],
    ['/model', handleModel],
    ['/showkey', handleShowkey],
    ['/clear', handleClear],
    ['/history', handleHistory],
    ['/export', handleExport],
    ['/import', handleImport],
    ['/theme', handleTheme],
    ['/preferences', handlePreferences],
    ['/prefs', handlePreferences],
    ['/compact', handleCompact],
    ['/batch', handleBatch],
    ['/copy', handleCopy],
    ['/clearall', handleClearall],
    ['/help', handleHelp],
]);

export async function handleSlashCommand(command, ctx) {
    const handler = COMMAND_REGISTRY.get(command) ?? handleHelp;
    return handler(ctx);
}
