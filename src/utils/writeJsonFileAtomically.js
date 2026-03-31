import fs from 'fs';
import path from 'path';

export function writeJsonFileAtomically(filePath, jsonData, fileWriteOptions = {}) {
    const json = JSON.stringify(jsonData, null, 2);
    const dir = path.dirname(filePath);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    const base = path.basename(filePath);
    const tmp = path.join(dir, `.${base}.${process.pid}.${Date.now()}.tmp`);
    fs.writeFileSync(tmp, json, { encoding: 'utf-8', ...fileWriteOptions });
    fs.renameSync(tmp, filePath);
}
