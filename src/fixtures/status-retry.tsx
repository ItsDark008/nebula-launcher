import { createRoot } from "react-dom/client";
import App from "../App";
import type { StatusClient } from "../status";
import type { AccountClient } from "../accounts";

const accounts: AccountClient = {
  list: async () => ({ active: null, accounts: [] }),
  signIn: async () => { throw "Not available in the fixture."; },
  select: async () => {},
  remove: async () => {},
  refresh: async () => { throw "Not available in the fixture."; },
};

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
      protectedAccess: "verified",
    };
  },
  async subscribe() {
    return () => {};
  },
};

createRoot(document.getElementById("root")!).render(<App client={client} accounts={accounts} />);
