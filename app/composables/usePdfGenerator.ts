import { computed, h, reactive } from "vue";
import { Document, Image, Page, Text, View, usePdf } from "@ceereals/vue-pdf";
import * as Logger from "@tauri-apps/plugin-log";

export function usePdfGenerator() {
  const { t, n, d } = useI18n();
  const { showErrorToast } = useCommandError();

  const config = reactive({
    marginTop: 40,
    marginX: 20,
    marginBottom: 40,
    vat: 20,
    template: {
      bytes: null as Uint8Array | null,
      name: null as string | null,
    },
    fields: {
      full_name: true,
      email: true,
      address: true,
      phone_number: true,
      status: true,
      vat: true,
    },
  });

  type DocType = "order" | "invoice" | "quote" | "delivery-note" | "credit-note";

  function toDataUri(bytes: Uint8Array, name?: string | null): string | null {
    const file = (name ?? "").toLowerCase();
    const isPng = file.endsWith(".png");
    const isJpg = file.endsWith(".jpg") || file.endsWith(".jpeg");
    if (!isPng && !isJpg) return null;
    const mime = isPng ? "image/png" : "image/jpeg";
    const binary = Array.from(bytes, (b) => String.fromCharCode(b)).join("");
    const base64 = btoa(binary);
    return `data:${mime};base64,${base64}`;
  }

  const styles = {
    page: (cfg: typeof config) => ({
      paddingTop: cfg.marginTop,
      paddingBottom: cfg.marginBottom,
      paddingHorizontal: cfg.marginX,
      fontSize: 10,
      color: "#0f172a",
    }),
    section: {
      marginBottom: 14,
    },
    title: {
      fontSize: 20,
      fontWeight: 700,
      marginBottom: 2,
    },
    muted: {
      fontSize: 8.5,
      color: "#475569",
    },
    label: {
      fontSize: 8,
      color: "#64748b",
      textTransform: "uppercase",
      letterSpacing: 0.6,
    },
    value: {
      fontSize: 9.5,
      color: "#0f172a",
      fontWeight: 500,
    },
    divider: {
      borderBottomWidth: 1,
      borderBottomColor: "#cbd5e1",
      marginTop: 8,
      marginBottom: 10,
    },
    cols: {
      flexDirection: "row",
      gap: 12,
      marginBottom: 10,
    },
    col: {
      width: "50%",
    },
    block: {
      borderWidth: 1,
      borderColor: "#e2e8f0",
      paddingTop: 8,
      paddingBottom: 6,
      paddingHorizontal: 8,
      borderRadius: 0,
      marginBottom: 10,
      backgroundColor: "#f8fafc",
      minHeight: 96,
    },
    sectionTitle: {
      fontSize: 8,
      color: "#64748b",
      textTransform: "uppercase",
      letterSpacing: 0.6,
      marginBottom: 6,
    },
    row: {
      flexDirection: "row",
      justifyContent: "space-between",
      alignItems: "center",
      marginBottom: 4,
    },
    tableHeader: {
      flexDirection: "row",
      borderBottomWidth: 1,
      borderBottomColor: "#94a3b8",
      borderTopWidth: 1,
      borderTopColor: "#94a3b8",
      paddingTop: 5,
      paddingBottom: 5,
      marginBottom: 2,
      fontSize: 8.5,
      fontWeight: 600,
      color: "#1e293b",
    },
    tableRow: {
      flexDirection: "row",
      borderBottomWidth: 1,
      borderBottomColor: "#e2e8f0",
      paddingVertical: 5,
      fontSize: 9,
    },
    cellName: {
      width: "42%",
    },
    cellQty: {
      width: "18%",
      textAlign: "right",
    },
    cellPrice: {
      width: "20%",
      textAlign: "right",
    },
    cellTotal: {
      width: "20%",
      textAlign: "right",
    },
    totals: {
      marginTop: 10,
      alignSelf: "flex-end",
      width: "50%",
      borderTopWidth: 1,
      borderTopColor: "#94a3b8",
      paddingTop: 7,
    },
    bgTemplate: {
      position: "absolute",
      left: 0,
      top: 0,
      width: "100%",
      height: "100%",
      zIndex: -1,
    },
  } as const;

  async function generatePdf(data: any, type: DocType) {
    if (!data) return "";
    try {
      const templateImage = config.template.bytes
        ? toDataUri(config.template.bytes, config.template.name)
        : null;
      const items = Array.isArray(data.items) ? data.items : [];
      const vatMultiplier = 1 + (Number(config.vat) || 0) / 100;
      const subtotal = Number(data.total ?? 0);
      const totalWithVat = config.fields.vat ? subtotal * vatMultiplier : subtotal;
      const statusLabel = data.status ? t(`status.${String(data.status).toLowerCase()}`) : "-";
      const createdAtLabel = data.created_at ? d(new Date(data.created_at), "long") : "-";

      const doc = h(
        Document,
        {
          title: data.identifier ?? type,
          author: "Mahalli",
          subject: type,
          creator: "Mahalli",
          producer: "Mahalli",
        },
        () => [
          h(Page, { size: "A4", style: styles.page(config) }, () => [
            templateImage ? h(Image, { src: templateImage, style: styles.bgTemplate }) : null,
            h(View, { style: styles.section }, () => [
              h(Text, { style: styles.muted }, t(`routes.${type}s`)),
              h(Text, { style: styles.title }, data.identifier ?? "-"),
              h(Text, { style: styles.muted }, createdAtLabel),
              h(View, { style: styles.divider }),
            ]),
            h(View, { style: styles.cols }, () => [
              h(View, { style: styles.col }, () => [
                h(View, { style: styles.block }, () => [
                  h(Text, { style: styles.sectionTitle }, t("fields.bill-to")),
                  h(View, { style: styles.row }, [
                    h(Text, { style: styles.label }, t("fields.full-name")),
                    h(Text, { style: styles.value }, data.client?.full_name ?? data.full_name ?? "-"),
                  ]),
                  config.fields.email
                    ? h(View, { style: styles.row }, [
                        h(Text, { style: styles.label }, t("fields.email")),
                        h(Text, { style: styles.value }, data.client?.email ?? data.email ?? "-"),
                      ])
                    : null,
                  config.fields.phone_number
                    ? h(View, { style: styles.row }, [
                        h(Text, { style: styles.label }, t("fields.phone")),
                        h(Text, { style: styles.value }, data.client?.phone_number ?? data.phone_number ?? "-"),
                      ])
                    : null,
                  config.fields.address
                    ? h(View, { style: styles.row }, [
                        h(Text, { style: styles.label }, t("fields.address")),
                        h(Text, { style: styles.value }, data.client?.address ?? data.address ?? "-"),
                      ])
                    : null,
                ]),
              ]),
              h(View, { style: styles.col }, () => [
                h(View, { style: styles.block }, () => [
                  h(Text, { style: styles.sectionTitle }, t("fields.configuration")),
                  config.fields.status
                    ? h(View, { style: styles.row }, [
                        h(Text, { style: styles.label }, t("fields.status")),
                        h(Text, { style: styles.value }, statusLabel),
                      ])
                    : null,
                  h(View, { style: styles.row }, [
                    h(Text, { style: styles.label }, t("fields.date")),
                    h(Text, { style: styles.value }, createdAtLabel),
                  ]),
                  h(View, { style: styles.row }, [
                    h(Text, { style: styles.label }, t("fields.currency")),
                    h(Text, { style: styles.value }, "MAD"),
                  ]),
                ]),
              ]),
            ]),
            h(View, {}, () => [
              h(View, { style: styles.tableHeader }, () => [
                h(Text, { style: styles.cellName }, t("fields.product")),
                h(Text, { style: styles.cellQty }, t("fields.quantity")),
                h(Text, { style: styles.cellPrice }, t("fields.price")),
                h(Text, { style: styles.cellTotal }, t("fields.total")),
              ]),
              ...items.map((item: any) => {
                const quantity = Number(item.quantity ?? 0);
                const price = Number(item.price ?? 0);
                const lineTotal = quantity * price;
                return h(View, { style: styles.tableRow }, () => [
                  h(Text, { style: styles.cellName }, item.name ?? "-"),
                  h(Text, { style: styles.cellQty }, String(quantity)),
                  h(Text, { style: styles.cellPrice }, n(price, "currency")),
                  h(Text, { style: styles.cellTotal }, n(lineTotal, "currency")),
                ]);
              }),
            ]),
            h(View, { style: styles.totals }, () => [
              h(View, { style: styles.row }, [
                h(Text, {}, t("fields.subtotal")),
                h(Text, {}, n(subtotal, "currency")),
              ]),
              config.fields.vat
                ? h(View, { style: styles.row }, [
                    h(Text, {}, `${t("fields.vat-rate")} (${config.vat}%)`),
                    h(Text, {}, n(subtotal * (Number(config.vat) / 100), "currency")),
                  ])
                : null,
              h(View, { style: styles.row }, [
                h(Text, {}, t("fields.total")),
                h(Text, {}, n(totalWithVat, "currency")),
              ]),
            ]),
          ]),
        ],
      );

      const pdf = await usePdf(computed(() => doc), { reactive: false });
      await pdf.execute(true);
      return pdf.url.value ?? "";
    } catch (err: any) {
      showErrorToast(err);
      Logger.error(`PDF GENERATION ERROR: ${err?.message ?? JSON.stringify(err)}`);
      return "";
    }
  }

  return {
    config,
    generatePdf,
  };
}
