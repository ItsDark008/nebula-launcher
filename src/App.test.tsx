import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, test, vi } from "vitest";
import App from "./App";
import type { MinecraftAccessStatus, StatusClient } from "./status";
import type { AccountClient } from "./accounts";

const accounts: AccountClient = {
  list: async () => ({ active: null, accounts: [] }),
  signIn: async () => ({ id: "fixture", name: "Player" }),
  select: async () => {}, remove: async () => {},
  refresh: async () => ({ id: "fixture", name: "Player" }),
};

const snapshot: MinecraftAccessStatus = {
  registration: "registered",
  reviewSubmission: "submitted",
  reviewApproval: "approved",
  protectedAccess: "verified",
};

afterEach(cleanup);

test("opens Dashboard and navigates to every destination", () => {
  render(<App accounts={accounts} client={{ read: async () => snapshot, subscribe: async () => () => {} }} />);
  expect(screen.getByRole("heading", { name: "Dashboard" })).toBeTruthy();
  for (const name of ["Instances", "Downloads", "Accounts", "Settings", "Diagnostics", "Dashboard"]) {
    const button = screen.getByRole("button", { name });
    fireEvent.click(button);
    expect(button.getAttribute("aria-current")).toBe("page");
    expect(screen.getByRole("heading", { name, level: 1 })).toBeTruthy();
  }
});

test("signs in and displays only the returned account identity", async () => {
  let signedIn = false;
  const accountFixture: AccountClient = {
    ...accounts,
    list: async () => signedIn ? { active: "fixture", accounts: [{ id: "fixture", name: "Player" }] } : { active: null, accounts: [] },
    signIn: async () => { signedIn = true; return { id: "fixture", name: "Player" }; },
  };
  render(<App accounts={accountFixture} client={{ read: async () => snapshot, subscribe: async () => () => {} }} />);
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  await screen.findByText("No accounts yet.");
  fireEvent.click(screen.getByRole("button", { name: "Sign in with Microsoft" }));
  await screen.findByText("Player (active)");
  expect(screen.getByRole("button", { name: "Refresh session" })).toBeTruthy();
});

test("does not render internal account errors", async () => {
  render(<App accounts={{ ...accounts, signIn: async () => { throw "private diagnostic"; } }} client={{ read: async () => snapshot, subscribe: async () => () => {} }} />);
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  await screen.findByText("No accounts yet.");
  fireEvent.click(screen.getByRole("button", { name: "Sign in with Microsoft" }));
  expect((await screen.findByText("Unable to update accounts. Try again.")).getAttribute("role")).toBe("alert");
  expect(screen.queryByText("private diagnostic")).toBeNull();
});

test("switches accounts and restores the remaining account after removal", async () => {
  let active = "a";
  let profiles = [{ id: "a", name: "A" }, { id: "b", name: "B" }];
  const refresh = vi.fn(async () => profiles.find(profile => profile.id === active)!);
  const accountFixture: AccountClient = {
    ...accounts,
    list: async () => ({ active, accounts: profiles }),
    select: async id => { active = id; },
    remove: async id => { profiles = profiles.filter(profile => profile.id !== id); if (active === id) active = profiles[0]?.id ?? ""; },
    refresh,
  };
  render(<App accounts={accountFixture} client={{ read: async () => snapshot, subscribe: async () => () => {} }} />);
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  await screen.findByText("A (active)");
  await waitFor(() => expect(refresh).toHaveBeenCalledTimes(1));
  await waitFor(() => expect((screen.getByRole("button", { name: "Select B" }) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(screen.getByRole("button", { name: "Select B" }));
  await screen.findByText("B (active)");
  expect(refresh).toHaveBeenCalledTimes(2);
  fireEvent.click(screen.getByRole("button", { name: "Remove B" }));
  await screen.findByText("A (active)");
  expect(refresh).toHaveBeenCalledTimes(3);
});

test("subscribes first and renders the command response even without an event", async () => {
  const dispose = vi.fn();
  const subscribe = vi.fn<StatusClient["subscribe"]>().mockResolvedValue(dispose);
  const read = vi.fn<StatusClient["read"]>().mockResolvedValue(snapshot);

  const { unmount } = render(<App accounts={accounts} client={{ read, subscribe }} />);
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  expect(screen.getByRole("status").textContent).toMatch(/refreshing/i);
  await screen.findByText("Protected access was verified with an eligible account.");
  expect(subscribe.mock.invocationCallOrder[0]).toBeLessThan(read.mock.invocationCallOrder[0]);

  unmount();
  expect(dispose).toHaveBeenCalledOnce();
});

test.each([{ code: "STATUS_UNAVAILABLE", message: "private error" }, "private error"])(
  "renders a safe failure and recovers on retry for %j",
  async (failure) => {
    const read = vi.fn<StatusClient["read"]>()
      .mockRejectedValueOnce(failure)
      .mockResolvedValueOnce(snapshot);
    render(<App accounts={accounts} client={{ read, subscribe: async () => () => {} }} />);
    fireEvent.click(screen.getByRole("button", { name: "Accounts" }));

    expect((await screen.findByRole("alert")).textContent).toBe("Unable to refresh launcher status.");
    expect(screen.queryByText(/private error/)).toBeNull();
    const retry = screen.getByRole("button", { name: "Retry" });
    retry.focus();
    fireEvent.click(retry);
    await screen.findByText("Protected access was verified with an eligible account.");
    expect(screen.queryByRole("alert")).toBeNull();
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Refresh status" }));
  },
);

test("disposes a subscription that finishes after unmount", async () => {
  const dispose = vi.fn();
  let finishSubscription!: (dispose: () => void) => void;
  const subscribe: StatusClient["subscribe"] = () => new Promise(resolve => {
    finishSubscription = resolve;
  });
  const read = vi.fn<StatusClient["read"]>();
  const { unmount } = render(<App accounts={accounts} client={{ read, subscribe }} />);
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  unmount();
  finishSubscription(dispose);

  await waitFor(() => expect(dispose).toHaveBeenCalledOnce());
  expect(read).not.toHaveBeenCalled();
});

test("handles an asynchronous unsubscribe failure during cleanup", async () => {
  const dispose = vi.fn().mockRejectedValue(new Error("subscription already closed"));
  const { unmount } = render(
    <App accounts={accounts} client={{ read: async () => snapshot, subscribe: async () => dispose }} />,
  );
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  await screen.findByText("Protected access was verified with an eligible account.");

  unmount();
  await waitFor(() => expect(dispose).toHaveBeenCalledOnce());
});
