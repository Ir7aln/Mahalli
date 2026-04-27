import type { Fail } from "@/bindings";
import { toast } from "vue-sonner";

type CommandError = Partial<Fail> & {
  code?: string | null;
  i18n_key?: string | null;
};

function isCommandError(value: unknown): value is CommandError {
  return typeof value === "object" && value !== null;
}

export function useCommandError() {
  const { t, te } = useI18n();

  function getErrorMessage(error: unknown, fallbackKey = "notifications.error.description") {
    if (isCommandError(error)) {
      if (error.i18n_key && te(error.i18n_key)) {
        return t(error.i18n_key);
      }

      if (typeof error.message === "string" && error.message.length > 0) {
        return error.message;
      }

      if (typeof error.error === "string" && error.error.length > 0) {
        return error.error;
      }
    }

    if (error instanceof Error && error.message) {
      return error.message;
    }

    if (typeof error === "string" && error.length > 0) {
      return error;
    }

    return t(fallbackKey);
  }

  function showErrorToast(
    error: unknown,
    options?: {
      titleKey?: string;
      fallbackKey?: string;
    },
  ) {
    toast.error(t(options?.titleKey ?? "notifications.error.title"), {
      description: getErrorMessage(error, options?.fallbackKey),
      closeButton: true,
    });
  }

  return {
    getErrorMessage,
    showErrorToast,
  };
}
