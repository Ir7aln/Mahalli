import {
  BaseDirectory,
  exists,
  mkdir,
  readFile,
  writeFile,
} from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import * as Logger from "@tauri-apps/plugin-log";

async function createFolder(folder: string) {
  if (!(await checkIfExistsInFs(folder))) {
    try {
      await mkdir(folder, {
        baseDir: BaseDirectory.AppData,
        recursive: true,
      });
      return true;
    } catch (err: any) {
      Logger.error(`CANT CREATE FOLDER :${err}`);
      return false;
    }
  }
  return true;
}

async function checkIfExistsInFs(fileOrFolder: string) {
  try {
    return await exists(fileOrFolder, {
      baseDir: BaseDirectory.AppData,
    });
  } catch (err: any) {
    Logger.error(`ERROR CHECKING IF EXISTS :${err}`);
    return false;
  }
}

export async function uploadFileToDataDir(
  folder: string,
  bytes: Uint8Array,
  name: string
) {
  try {
    await createFolder(folder);
    const path = await join(await appDataDir(), folder, name);
    await writeFile(path, bytes, {
      baseDir: BaseDirectory.AppData,
    });
    return path;
  } catch (err: any) {
    Logger.error(`ERROR UPLOADING FILE TO APP DATA DIR : ${err}`);
    return null;
  }
}

export async function getFileBytes(path?: string) {
  if (!path) 
    return null;
  try {
    const content = await readFile(path, {
      baseDir: BaseDirectory.Home,
    });
    return content;
  } catch (err: any) {
    Logger.error(`ERROR READING BINARY FILE :${err}`);
    return null;
  }
}

export function getBytesArray(file: File): Promise<Uint8Array> {
  const fileData = new Blob([file]);
  return new Promise((resolve) => {
    const reader = new FileReader();
    reader.readAsArrayBuffer(fileData);
    reader.onload = () => {
      const arrayBuffer = new Uint8Array(reader.result as ArrayBuffer);
      resolve(arrayBuffer);
    };
  });
}
