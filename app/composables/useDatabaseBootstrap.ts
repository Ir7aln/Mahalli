import * as Logger from "@tauri-apps/plugin-log";
import { commands, type DatabaseBootstrapStatus } from "@/bindings";

const emptyStatus: DatabaseBootstrapStatus = {
  databases: [],
  active_database: null,
  has_any_database: false,
  has_active_database: false,
};

export function useDatabaseBootstrap() {
  const status = useState<DatabaseBootstrapStatus>("database-bootstrap-status", () => emptyStatus);
  const pending = useState<boolean>("database-bootstrap-pending", () => false);
  const loaded = useState<boolean>("database-bootstrap-loaded", () => false);

  async function refreshStatus() {
    pending.value = true;

    try {
      const result = await commands.getDatabaseBootstrapStatus();

      if (result.status === "error") {
        Logger.error(`GET DATABASE BOOTSTRAP STATUS: ${JSON.stringify(result.error)}`);
        return status.value;
      }

      status.value = result.data.data ?? emptyStatus;
      loaded.value = true;
      return status.value;
    } finally {
      pending.value = false;
    }
  }

  return {
    status,
    pending,
    loaded,
    refreshStatus,
  };
}
