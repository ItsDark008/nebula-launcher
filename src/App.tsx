import { useEffect, useRef, useState } from "react";
import { statusClient, type MinecraftAccessStatus, type StatusClient } from "./status";
import "./styles.css";

interface StatusSession {
  refresh(): void;
}

function Accounts({ client }: { client: StatusClient }) {
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
      {status?.reviewApproval === "approved" && <p>Minecraft AppID review approved.</p>}
      {status?.protectedAccess === "unverified" && <p>Protected access has not been verified yet.</p>}
      {loading && <p role="status">Refreshing launcher status…</p>}
      {failed && <p role="alert">Unable to refresh launcher status.</p>}
      <button type="button" aria-disabled={loading} onClick={() => session.current?.refresh()}>
        {failed ? "Retry" : "Refresh status"}
      </button>
    </section>
  );
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

export default function App({ client = statusClient }: { client?: StatusClient }) {
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
            <Accounts client={client} />
          ) : (
            <p>{emptyContent[destination]}</p>
          )}
        </div>
      </main>
    </div>
  );
}
