import { useEffect, useRef, useState } from "react";
import { statusClient, type MinecraftAccessStatus, type StatusClient } from "./status";
import { accountClient, type AccountClient, type AccountList } from "./accounts";
import "./styles.css";

interface StatusSession {
  refresh(): void;
}

function Accounts({ client, accounts }: { client: StatusClient; accounts: AccountClient }) {
  const [status, setStatus] = useState<MinecraftAccessStatus>();
  const [loading, setLoading] = useState(true);
  const [failed, setFailed] = useState(false);
  const session = useRef<StatusSession | null>(null);

  useEffect(() => {
    let active = true;
    let pending = true;
    let unsubscribe: (() => void | Promise<void>) | undefined;
    setLoading(true);

    async function disposeSubscription(dispose: () => void | Promise<void>) {
      try {
        await dispose();
      } catch {
        // This view is already detached from the subscription.
      }
    }

    async function read() {
      setFailed(false);
      try {
        const snapshot = await client.read();
        if (active) setStatus(snapshot);
      } catch {
        if (active) setFailed(true);
      } finally {
        pending = false;
        if (active) setLoading(false);
      }
    }

    session.current = {
      refresh() {
        if (pending) return;
        pending = true;
        setLoading(true);
        void read();
      },
    };

    async function initialize() {
      try {
        const dispose = await client.subscribe((snapshot) => {
          if (active && !pending) setStatus(snapshot);
        });
        if (active) unsubscribe = dispose;
        else void disposeSubscription(dispose);
      } catch {
        // The command response can still supply the status.
      }
      if (active) await read();
    }

    void initialize();
    return () => {
      active = false;
      if (unsubscribe) void disposeSubscription(unsubscribe);
      session.current = null;
    };
  }, [client]);

  return (
    <section aria-labelledby="accounts-heading">
      <h2 id="accounts-heading">Minecraft access</h2>
      <AccountManager client={accounts} />
      {status?.reviewApproval === "approved" && <p>Minecraft AppID review approved.</p>}
      {status?.protectedAccess === "verified" && <p>Protected access was verified with an eligible account.</p>}
      {loading && <p role="status">Refreshing launcher status…</p>}
      {failed && <p role="alert">Unable to refresh launcher status.</p>}
      <button type="button" aria-disabled={loading} onClick={() => session.current?.refresh()}>
        {failed ? "Retry" : "Refresh status"}
      </button>
    </section>
  );
}

function AccountManager({ client }: { client: AccountClient }) {
  const [list, setList] = useState<AccountList>();
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");

  useEffect(() => {
    let active = true;
    client.list().then(async value => {
      if (!active) return;
      setList(value);
      if (value.active) {
        setBusy(true); setMessage("Restoring session…");
        try { await client.refresh(); }
        catch (failure) { if (active) setError(safeAccountError(failure)); }
        finally { if (active) { setBusy(false); setMessage(""); } }
      }
    }).catch((failure: unknown) => {
      if (active) setError(safeAccountError(failure));
    });
    return () => { active = false; };
  }, [client]);

  async function run(action: () => Promise<unknown>, progress: string) {
    if (busy) return;
    setBusy(true); setError(""); setMessage(progress);
    try {
      await action();
      setList(await client.list());
      setMessage("");
    } catch (failure) {
      try { setList(await client.list()); } catch { /* Keep the last known account list. */ }
      setError(safeAccountError(failure));
      setMessage("");
    } finally { setBusy(false); }
  }

  return <div className="account-manager">
    {message && <p role="status">{message}</p>}
    {error && <p role="alert">{error}</p>}
    {!list && error && <button type="button" disabled={busy} onClick={() => void run(async () => {}, "Checking accounts…")}>Retry accounts</button>}
    {list?.accounts.length === 0 && <p>No accounts yet.</p>}
    {list?.accounts.map(account => <div key={account.id}>
      <p>{account.name}{account.id === list.active ? " (active)" : ""}</p>
      {account.id !== list.active && <button type="button" disabled={busy} onClick={() => void run(async () => { await client.select(account.id); await client.refresh(); }, "Switching account…")}>Select {account.name}</button>}
      <button type="button" disabled={busy} onClick={() => void run(async () => {
        await client.remove(account.id);
        if (account.id === list.active && (await client.list()).active) await client.refresh();
      }, "Removing account…")}>Remove {account.name}</button>
    </div>)}
    <button type="button" disabled={busy} onClick={() => void run(() => client.signIn(), "Complete sign-in in your browser…")}>Sign in with Microsoft</button>
    {list?.active && <button type="button" disabled={busy} onClick={() => void run(() => client.refresh(), "Restoring session…")}>Refresh session</button>}
  </div>;
}

function safeAccountError(failure: unknown) {
  const allowed = ["The system keyring is unavailable or locked.", "The system keyring is locked or access was denied.", "The system keyring is unavailable.", "The stored account is unavailable.", "Could not open the system browser.", "Microsoft sign-in failed or expired. Try again.", "Xbox sign-in failed. Check your Xbox account and try again.", "Minecraft access or ownership could not be verified.", "Microsoft sign-in is not configured.", "Unable to restore this session. Check your connection and retry.", "The session expired or was revoked. Sign in again."];
  return typeof failure === "string" && allowed.includes(failure) ? failure : "Unable to update accounts. Try again.";
}

const destinations = ["Dashboard", "Instances", "Downloads", "Accounts", "Settings", "Diagnostics"] as const;
type Destination = typeof destinations[number];

const emptyContent: Record<Exclude<Destination, "Accounts">, string> = {
  Dashboard: "Your launcher workspace is ready. Explore the sections to get started.",
  Instances: "No instances yet. Instance management will be available in a later update.",
  Downloads: "No downloads yet. Downloads will appear here when available.",
  Settings: "Settings will be available in a later update.",
  Diagnostics: "Diagnostics will be available in a later update.",
};

export default function App({ client = statusClient, accounts = accountClient }: { client?: StatusClient; accounts?: AccountClient }) {
  const [destination, setDestination] = useState<Destination>("Dashboard");

  return (
    <div className="workspace">
      <aside className="sidebar">
        <div className="brand"><span aria-hidden="true">✦</span> Nebula</div>
        <nav aria-label="Main navigation">
          {destinations.map((name) => (
            <button
              key={name}
              type="button"
              className="nav-item"
              aria-current={destination === name ? "page" : undefined}
              onClick={() => setDestination(name)}
            >
              {name}
            </button>
          ))}
        </nav>
      </aside>
      <main className="workspace-main" tabIndex={-1}>
        <div className="eyebrow">NEBULA LAUNCHER</div>
        <h1>{destination}</h1>
        <div className="content-panel">
          {destination === "Accounts" ? (
            <Accounts client={client} accounts={accounts} />
          ) : (
            <p>{emptyContent[destination]}</p>
          )}
        </div>
      </main>
    </div>
  );
}
