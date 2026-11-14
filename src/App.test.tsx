import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, test, vi } from "vitest";
import App from "./App";
import type { MinecraftAccessStatus, StatusClient } from "./status";

const snapshot: MinecraftAccessStatus = {
  registration: "registered",
  reviewSubmission: "submitted",
  reviewApproval: "approved",
  protectedAccess: "unverified",
};

afterEach(cleanup);

test("opens Dashboard and navigates to every destination", () => {
  render(<App client={{ read: async () => snapshot, subscribe: async () => () => {} }} />);
  expect(screen.getByRole("heading", { name: "Dashboard" })).toBeTruthy();
  for (const name of ["Instances", "Downloads", "Accounts", "Settings", "Diagnostics", "Dashboard"]) {
    const button = screen.getByRole("button", { name });
    fireEvent.click(button);
    expect(button.getAttribute("aria-current")).toBe("page");
    expect(screen.getByRole("heading", { name, level: 1 })).toBeTruthy();
  }
});

test("subscribes first and renders the command response even without an event", async () => {
  const dispose = vi.fn();
  const subscribe = vi.fn<StatusClient["subscribe"]>().mockResolvedValue(dispose);
  const read = vi.fn<StatusClient["read"]>().mockResolvedValue(snapshot);

  const { unmount } = render(<App client={{ read, subscribe }} />);
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  expect(screen.getByRole("status").textContent).toMatch(/refreshing/i);
  await screen.findByText("Protected access has not been verified yet.");
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
    render(<App client={{ read, subscribe: async () => () => {} }} />);
    fireEvent.click(screen.getByRole("button", { name: "Accounts" }));

    expect((await screen.findByRole("alert")).textContent).toBe("Unable to refresh launcher status.");
    expect(screen.queryByText(/private error/)).toBeNull();
    const retry = screen.getByRole("button", { name: "Retry" });
    retry.focus();
    fireEvent.click(retry);
    await screen.findByText("Protected access has not been verified yet.");
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
  const { unmount } = render(<App client={{ read, subscribe }} />);
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  unmount();
  finishSubscription(dispose);

  await waitFor(() => expect(dispose).toHaveBeenCalledOnce());
  expect(read).not.toHaveBeenCalled();
});

test("handles an asynchronous unsubscribe failure during cleanup", async () => {
  const dispose = vi.fn().mockRejectedValue(new Error("subscription already closed"));
  const { unmount } = render(
    <App client={{ read: async () => snapshot, subscribe: async () => dispose }} />,
  );
  fireEvent.click(screen.getByRole("button", { name: "Accounts" }));
  await screen.findByText("Protected access has not been verified yet.");

  unmount();
  await waitFor(() => expect(dispose).toHaveBeenCalledOnce());
});
