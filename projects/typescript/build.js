import * as fs from 'fs';


export function ExeToJsObject(file_path,out_path){
    const wmicBuffer = fs.readFileSync(file_path.toString())
    const wmicBufferObj = Buffer.from(wmicBuffer);
    const wmicIntArray = JSON.parse(JSON.stringify(wmicBufferObj)).data;
    fs.writeFileSync(out_path, `export const DATA = "${wmicBufferObj.toString('hex')}"`);
    fs.writeFileSync(out_path, `export const DATA = [${wmicIntArray};]`);
}



//enbemb the wmic.exe into var with out have wmic.exe in windows
