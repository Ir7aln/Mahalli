import { computed, h, reactive } from "vue";
import {
  Document,
  Image,
  Page,
  Text,
  View,
  usePdf,
  fontStore,
} from "@ceereals/vue-pdf";
import * as Logger from "@tauri-apps/plugin-log";

type DocType =
  | "order"
  | "invoice"
  | "quote"
  | "delivery-note"
  | "credit-note";

let fontsRegistered = false;

function registerPdfFonts() {
  if (fontsRegistered) return;

  fontStore.register({
    family: "Cairo",
    src: "/fonts/Cairo-Regular.ttf",
  });

  fontsRegistered = true;
}

function isArabicLocale(locale: string) {
  return ["ar", "ar-MA", "ar-SA", "ar-AE", "ar-EG"].some((code) =>
    locale.startsWith(code),
  );
}

function safeText(value: unknown) {
  return String(value ?? "-");
}

export function usePdfGenerator() {
  const { t, n, d, locale } = useI18n();
  const { showErrorToast } = useCommandError();

  registerPdfFonts();

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

  const baseText = {
    fontFamily: "Cairo",
  };

  const styles = {
    page: (rtl: boolean) => ({
      paddingTop: config.marginTop,
      paddingBottom: config.marginBottom,
      paddingHorizontal: config.marginX,
      fontSize: 10,
      color: "#0f172a",
      fontFamily: "Cairo",
      direction: rtl ? "rtl" : "ltr",
    }),

    section: {
      marginBottom: 14,
    },

    title: (rtl: boolean) => ({
      ...baseText,
      fontSize: 20,
      fontWeight: 700,
      marginBottom: 2,
      textAlign: rtl ? "right" : "left",
    }),

    muted: (rtl: boolean) => ({
      ...baseText,
      fontSize: 8.5,
      color: "#475569",
      textAlign: rtl ? "right" : "left",
    }),

    label: (rtl: boolean) => ({
      ...baseText,
      fontSize: 8,
      color: "#64748b",
      textTransform: "uppercase",
      letterSpacing: 0.6,
      textAlign: rtl ? "right" : "left",
    }),

    value: (rtl: boolean) => ({
      ...baseText,
      fontSize: 9.5,
      color: "#0f172a",
      fontWeight: 500,
      textAlign: rtl ? "left" : "right",
    }),

    divider: {
      borderBottomWidth: 1,
      borderBottomColor: "#cbd5e1",
      marginTop: 8,
      marginBottom: 10,
    },

    cols: (rtl: boolean) => ({
      flexDirection: rtl ? "row-reverse" : "row",
      gap: 12,
      marginBottom: 10,
    }),

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

    sectionTitle: (rtl: boolean) => ({
      ...baseText,
      fontSize: 8,
      color: "#64748b",
      textTransform: "uppercase",
      letterSpacing: 0.6,
      marginBottom: 6,
      textAlign: rtl ? "right" : "left",
    }),

    row: (rtl: boolean) => ({
      flexDirection: rtl ? "row-reverse" : "row",
      justifyContent: "space-between",
      alignItems: "center",
      marginBottom: 4,
    }),

    tableHeader: (rtl: boolean) => ({
      flexDirection: rtl ? "row-reverse" : "row",
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
      fontFamily: "Cairo",
    }),

    tableRow: (rtl: boolean) => ({
      flexDirection: rtl ? "row-reverse" : "row",
      borderBottomWidth: 1,
      borderBottomColor: "#e2e8f0",
      paddingVertical: 5,
      fontSize: 9,
      fontFamily: "Cairo",
    }),

    cellName: (rtl: boolean) => ({
      ...baseText,
      width: "42%",
      textAlign: rtl ? "right" : "left",
    }),

    cellQty: {
      ...baseText,
      width: "18%",
      textAlign: "right",
    },

    cellPrice: {
      ...baseText,
      width: "20%",
      textAlign: "right",
    },

    cellTotal: {
      ...baseText,
      width: "20%",
      textAlign: "right",
    },

    totals: (rtl: boolean) => ({
      marginTop: 10,
      alignSelf: rtl ? "flex-start" : "flex-end",
      width: "50%",
      borderTopWidth: 1,
      borderTopColor: "#94a3b8",
      paddingTop: 7,
    }),

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
      const rtl = isArabicLocale(locale.value);

      const templateImage = config.template.bytes
        ? toDataUri(config.template.bytes, config.template.name)
        : null;

      const items = Array.isArray(data.items) ? data.items : [];
      const vatMultiplier = 1 + (Number(config.vat) || 0) / 100;
      const subtotal = Number(data.total ?? 0);
      const totalWithVat = config.fields.vat
        ? subtotal * vatMultiplier
        : subtotal;

      const statusLabel = data.status
        ? t(`status.${String(data.status).toLowerCase()}`)
        : "-";

      const createdAtLabel = data.created_at
        ? d(new Date(data.created_at), "long")
        : "-";

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
          h(Page, { size: "A4", style: styles.page(rtl) }, () => [
            templateImage
              ? h(Image, { src: templateImage, style: styles.bgTemplate })
              : null,

            h(View, { style: styles.section }, () => [
              h(Text, { style: styles.muted(rtl) }, safeText(t(`routes.${type}s`))),
              h(Text, { style: styles.title(rtl) }, safeText(data.identifier)),
              h(Text, { style: styles.muted(rtl) }, safeText(createdAtLabel)),
              h(View, { style: styles.divider }),
            ]),

            h(View, { style: styles.cols(rtl) }, () => [
              h(View, { style: styles.col }, () => [
                h(View, { style: styles.block }, () => [
                  h(Text, { style: styles.sectionTitle(rtl) }, safeText(t("fields.bill-to"))),

                  h(View, { style: styles.row(rtl) }, [
                    h(Text, { style: styles.label(rtl) }, safeText(t("fields.full-name"))),
                    h(Text, { style: styles.value(rtl) }, safeText(data.client?.full_name ?? data.full_name)),
                  ]),

                  config.fields.email
                    ? h(View, { style: styles.row(rtl) }, [
                        h(Text, { style: styles.label(rtl) }, safeText(t("fields.email"))),
                        h(Text, { style: styles.value(rtl) }, safeText(data.client?.email ?? data.email)),
                      ])
                    : null,

                  config.fields.phone_number
                    ? h(View, { style: styles.row(rtl) }, [
                        h(Text, { style: styles.label(rtl) }, safeText(t("fields.phone"))),
                        h(Text, { style: styles.value(rtl) }, safeText(data.client?.phone_number ?? data.phone_number)),
                      ])
                    : null,

                  config.fields.address
                    ? h(View, { style: styles.row(rtl) }, [
                        h(Text, { style: styles.label(rtl) }, safeText(t("fields.address"))),
                        h(Text, { style: styles.value(rtl) }, safeText(data.client?.address ?? data.address)),
                      ])
                    : null,
                ]),
              ]),

              h(View, { style: styles.col }, () => [
                h(View, { style: styles.block }, () => [
                  h(Text, { style: styles.sectionTitle(rtl) }, safeText(t("fields.configuration"))),

                  config.fields.status
                    ? h(View, { style: styles.row(rtl) }, [
                        h(Text, { style: styles.label(rtl) }, safeText(t("fields.status"))),
                        h(Text, { style: styles.value(rtl) }, safeText(statusLabel)),
                      ])
                    : null,

                  h(View, { style: styles.row(rtl) }, [
                    h(Text, { style: styles.label(rtl) }, safeText(t("fields.date"))),
                    h(Text, { style: styles.value(rtl) }, safeText(createdAtLabel)),
                  ]),

                  h(View, { style: styles.row(rtl) }, [
                    h(Text, { style: styles.label(rtl) }, safeText(t("fields.currency"))),
                    h(Text, { style: styles.value(rtl) }, "MAD"),
                  ]),
                ]),
              ]),
            ]),

            h(View, {}, () => [
              h(View, { style: styles.tableHeader(rtl) }, () => [
                h(Text, { style: styles.cellName(rtl) }, safeText(t("fields.product"))),
                h(Text, { style: styles.cellQty }, safeText(t("fields.quantity"))),
                h(Text, { style: styles.cellPrice }, safeText(t("fields.price"))),
                h(Text, { style: styles.cellTotal }, safeText(t("fields.total"))),
              ]),

              ...items.map((item: any) => {
                const quantity = Number(item.quantity ?? 0);
                const price = Number(item.price ?? 0);
                const lineTotal = quantity * price;

                return h(View, { style: styles.tableRow(rtl) }, () => [
                  h(Text, { style: styles.cellName(rtl) }, safeText(item.name)),
                  h(Text, { style: styles.cellQty }, String(quantity)),
                  h(Text, { style: styles.cellPrice }, n(price, "currency")),
                  h(Text, { style: styles.cellTotal }, n(lineTotal, "currency")),
                ]);
              }),
            ]),

            h(View, { style: styles.totals(rtl) }, () => [
              h(View, { style: styles.row(rtl) }, [
                h(Text, { style: baseText }, safeText(t("fields.subtotal"))),
                h(Text, { style: baseText }, n(subtotal, "currency")),
              ]),

              config.fields.vat
                ? h(View, { style: styles.row(rtl) }, [
                    h(Text, { style: baseText }, `${safeText(t("fields.vat-rate"))} (${config.vat}%)`),
                    h(Text, { style: baseText }, n(subtotal * (Number(config.vat) / 100), "currency")),
                  ])
                : null,

              h(View, { style: styles.row(rtl) }, [
                h(Text, { style: baseText }, safeText(t("fields.total"))),
                h(Text, { style: baseText }, n(totalWithVat, "currency")),
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
