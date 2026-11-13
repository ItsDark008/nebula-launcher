import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface MinecraftAccessStatus {
  registration: "awaiting_owner_registration";
  reviewSubmission: "pending";
  reviewApproval: "pending";
  protectedAccess: "blocked";
}

export interface StatusClient {
  read(): Promise<MinecraftAccessStatus>;
  subscribe(callback: (status: MinecraftAccessStatus) => void): Promise<() => void | Promise<void>>;
}

export const statusClient: StatusClient = {
  read: () => invoke<MinecraftAccessStatus>("get_launcher_status"),
  subscribe: (callback) =>
    listen<MinecraftAccessStatus>("launcher-status-updated", ({ payload }) => callback(payload)),
};
