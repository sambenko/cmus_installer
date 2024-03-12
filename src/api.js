import { invoke } from '@tauri-apps/api/tauri';

export async function downloadCmus(version, targetPath) {
  try {
    await invoke('download_cmus', { version, targetPath });
    return true;
  } catch (error) {
    console.error('Error downloading cmus:', error);
    return false;
  }
}

export async function decompress(sourcePath, targetPath) {
  try {
    await invoke('decompress', { sourcePath, targetPath });
    return true;
  } catch (error) {
    console.error('Error decompressing file:', error);
    return false;
  }
}

export async function installCmus(targetPath) {
  try {
    await invoke('install_cmus', { targetPath });
    return true;
  } catch (error) {
    console.error('Error installing cmus:', error);
    return false;
  }
}