import { invoke } from "@tauri-apps/api/core";
import type { Result } from "../types";

async function WriteConfig(config_name: string, database_id: string, token: string) {
    return invoke("write_config", { configName: config_name, databaseId: database_id, token });
}

async function LoadOptions() : Promise<[string[], string]> {
    return invoke("get_options");
}

async function Crawl(target: string): Promise<string> {
    return invoke("crawl_douban_movie", { target });
}

async function PickOption(index: string) {
    return invoke("pick_option", { index });
}

async function QueryMovie(name: string): Promise<Result[]> {
    return invoke("query_movie", { name });
}

async function AddToPath(installDir: string): Promise<void> {
    return invoke("add_to_path", { installDir });
}

async function RemoveFromPath(installDir: string): Promise<void> {
    return invoke("remove_from_path", { installDir });
}

async function IsPathAdded(installDir: string): Promise<boolean> {
    return invoke("is_path_added", { installDir });
}

async function GetInstallDir(): Promise<string> {
    return invoke("get_install_dir");
}


export { LoadOptions, Crawl, WriteConfig, PickOption, QueryMovie, AddToPath, RemoveFromPath, IsPathAdded, GetInstallDir }
