import { invoke } from '@tauri-apps/api/tauri';

export function get_download_links(): Promise<string> {
    return invoke("get_download_links");
}

export function download(url: string): Promise<void> {
    return invoke("download", { url });
}

export function list_dirs(): Promise<DirEntry[]> {
    return invoke("list_dirs", { path: "/home/miguel/blenders" });
}

export function extract_tar_xz(tarPath: string): Promise<void> {
    return invoke("extract_tar_xz", { tarPath });
}

export function remove_file(filePath: string): Promise<void> {
    return invoke("remove_file", { filePath });
}

export function remove_dir(dirPath: string): Promise<void> {
    return invoke("remove_dir", { dirPath });
}