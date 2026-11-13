import { useEffect, useRef, useState } from "react";
import { statusClient, type MinecraftAccessStatus, type StatusClient } from "./status";
import "./styles.css";

interface StatusSession {
  refresh(): void;
}

export default function App({ client = statusClient }: { client?: StatusClient }) {
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
    <main>
      <h1>Minecraft access</h1>
      {status?.protectedAccess === "blocked" && <p>Protected access is blocked.</p>}
      {status?.registration === "awaiting_owner_registration" && (
        <p>Waiting for the project owner to register Nebula Launcher.</p>
      )}
      {loading && <p role="status">Refreshing launcher status…</p>}
      {failed && <p role="alert">Unable to refresh launcher status.</p>}
      <button type="button" aria-disabled={loading} onClick={() => session.current?.refresh()}>
        {failed ? "Retry" : "Refresh status"}
      </button>
    </main>
  );
}
