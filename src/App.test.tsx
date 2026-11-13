import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, test, vi } from "vitest";
import App from "./App";
import type { MinecraftAccessStatus, StatusClient } from "./status";

const snapshot: MinecraftAccessStatus = {
  registration: "awaiting_owner_registration",
  reviewSubmission: "pending",
  reviewApproval: "pending",
  protectedAccess: "blocked",
};

afterEach(cleanup);

test("subscribes first and renders the command response even without an event", async () => {
  const dispose = vi.fn();
  const subscribe = vi.fn<StatusClient["subscribe"]>().mockResolvedValue(dispose);
  const read = vi.fn<StatusClient["read"]>().mockResolvedValue(snapshot);

  const { unmount } = render(<App client={{ read, subscribe }} />);
  expect(screen.getByRole("status").textContent).toMatch(/refreshing/i);
  await screen.findByText("Protected access is blocked.");
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

    expect((await screen.findByRole("alert")).textContent).toBe("Unable to refresh launcher status.");
    expect(screen.queryByText(/private error/)).toBeNull();
    const retry = screen.getByRole("button", { name: "Retry" });
    retry.focus();
    fireEvent.click(retry);
    await screen.findByText("Protected access is blocked.");
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
  await screen.findByText("Protected access is blocked.");

  unmount();
  await waitFor(() => expect(dispose).toHaveBeenCalledOnce());
});
