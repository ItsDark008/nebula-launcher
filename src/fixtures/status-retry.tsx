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
      registration: "registered",
      reviewSubmission: "submitted",
      reviewApproval: "approved",
      protectedAccess: "unverified",
    };
  },
  async subscribe() {
    return () => {};
  },
};

createRoot(document.getElementById("root")!).render(<App client={client} />);
