import { invoke } from "@tauri-apps/api/core";

export interface Account { id: string; name: string }
export interface AccountList { active: string | null; accounts: Account[] }
export interface AccountClient {
  list(): Promise<AccountList>;
  signIn(): Promise<Account>;
  select(id: string): Promise<void>;
  remove(id: string): Promise<void>;
  refresh(): Promise<Account>;
}
export const accountClient: AccountClient = {
  list: () => invoke("list_accounts"),
  signIn: () => invoke("sign_in_account"),
  select: (id) => invoke("select_account", { id }),
  remove: (id) => invoke("remove_account", { id }),
  refresh: () => invoke("refresh_account"),
};
