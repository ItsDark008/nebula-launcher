import { createRoot } from "react-dom/client";
import App from "../App";
import type { StatusClient } from "../status";

let firstRead = true;
const client: StatusClient = {
  async read() {
    if (firstRead) {
      firstRead = false;
      throw { code: "STATUS_UNAVAILABLE", message: "Synthetic private diagnostic" };
    }
    return {
      registration: "awaiting_owner_registration",
      reviewSubmission: "pending",
      reviewApproval: "pending",
      protectedAccess: "blocked",
    };
  },
  async subscribe() {
    return () => {};
  },
};

createRoot(document.getElementById("root")!).render(<App client={client} />);
