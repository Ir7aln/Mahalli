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
      paddingTop: 30,
      paddingBottom: 30,
      paddingHorizontal: 28,
      fontSize: 10,
      color: "#111827",
      fontFamily: "Cairo",
      direction: (rtl ? "rtl" : "ltr") as "rtl" | "ltr",
      backgroundColor: "#ffffff",
    }),

    header: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      justifyContent: "space-between" as const,
      alignItems: "center" as const,
      marginBottom: 18,
    }),

    title: (rtl: boolean) => ({
      fontSize: 28,
      fontWeight: 600,
      color: "#111827",
      textAlign: (rtl ? "right" : "left") as "right" | "left",
      letterSpacing: 1,
    }),

    logo: {
      width: 90,
      height: "auto",
    },

    section: {
      marginBottom: 18,
    },

    sectionTitle: (rtl: boolean) => ({
      fontSize: 12,
      fontWeight: 600,
      color: "#1f2937",
      borderBottomWidth: 1,
      borderBottomColor: "#e5e7eb",
      paddingBottom: 4,
      marginBottom: 8,
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    text: (rtl: boolean) => ({
      fontSize: 10,
      color: "#374151",
      marginBottom: 3,
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    muted: (rtl: boolean) => ({
      fontSize: 9,
      color: "#6b7280",
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    grid: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      gap: 20,
      marginBottom: 12,
    }),

    col: {
      width: "50%",
    },

    tableHeader: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      paddingBottom: 6,
      marginBottom: 4,
      borderBottomWidth: 1,
      borderBottomColor: "#d1d5db",
      fontSize: 10,
      fontWeight: 600,
      color: "#111827",
    }),

    tableRow: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      paddingVertical: 6,
      fontSize: 10,
      color: "#374151",
    }),

    cellName: (rtl: boolean) => ({
      width: "45%",
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    cellQty: {
      width: "15%",
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

    totals: (rtl: boolean) => ({
      marginTop: 20,
      alignSelf: (rtl ? "flex-start" : "flex-end") as "flex-start" | "flex-end",
      width: "45%",
    }),

    totalRow: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      justifyContent: "space-between" as const,
      marginBottom: 6,
      fontSize: 11,
    }),

    totalFinal: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      justifyContent: "space-between" as const,
      marginTop: 8,
      paddingTop: 6,
      borderTopWidth: 1,
      borderTopColor: "#9ca3af",
      fontSize: 12,
      fontWeight: 600,
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
        },
        () => [
          h(Page, { size: "A4", style: styles.page(rtl) }, () => [
            templateImage
              ? h(Image, { src: templateImage, style: styles.bgTemplate })
              : null,

            // 🔥 HEADER
            h(View, { style: styles.header(rtl) }, () => [
              h(View, {}, () => [
                h(Text, { style: styles.title(rtl) }, type.toUpperCase()),
                h(Text, { style: styles.muted(rtl) }, safeText(data.identifier)),
                h(Text, { style: styles.muted(rtl) }, safeText(createdAtLabel)),
              ]),

              templateImage
                ? h(Image, { src: templateImage, style: styles.logo })
                : null,
            ]),

            // 🔥 CUSTOMER + META GRID
            h(View, { style: styles.grid(rtl) }, () => [
              // LEFT - CUSTOMER
              h(View, { style: styles.col }, () => [
                h(Text, { style: styles.sectionTitle(rtl) }, t("fields.bill-to")),

                h(Text, { style: styles.text(rtl) },
                  safeText(data.client?.full_name ?? data.full_name)
                ),

                config.fields.email
                  ? h(Text, { style: styles.text(rtl) },
                      safeText(data.client?.email ?? data.email))
                  : null,

                config.fields.phone_number
                  ? h(Text, { style: styles.text(rtl) },
                      safeText(data.client?.phone_number ?? data.phone_number))
                  : null,

                config.fields.address
                  ? h(Text, { style: styles.text(rtl) },
                      safeText(data.client?.address ?? data.address))
                  : null,
              ]),

              // RIGHT - META
              h(View, { style: styles.col }, () => [
                h(Text, { style: styles.sectionTitle(rtl) }, t("fields.configuration")),

                config.fields.status
                  ? h(Text, { style: styles.text(rtl) },
                      `${t("fields.status")}: ${safeText(statusLabel)}`)
                  : null,

                h(Text, { style: styles.text(rtl) },
                  `${t("fields.date")}: ${safeText(createdAtLabel)}`),

                h(Text, { style: styles.text(rtl) }, "MAD"),
              ]),
            ]),

            // 🔥 TABLE
            h(View, { style: styles.section }, () => [
              // HEADER
              h(View, { style: styles.tableHeader(rtl) }, () => [
                h(Text, { style: styles.cellName(rtl) }, t("fields.product")),
                h(Text, { style: styles.cellQty }, t("fields.quantity")),
                h(Text, { style: styles.cellPrice }, t("fields.price")),
                h(Text, { style: styles.cellTotal }, t("fields.total")),
              ]),

              // ROWS
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

            // 🔥 TOTALS
            h(View, { style: styles.totals(rtl) }, () => [
              h(View, { style: styles.totalRow(rtl) }, [
                h(Text, {}, t("fields.subtotal")),
                h(Text, {}, n(subtotal, "currency")),
              ]),

              config.fields.vat
                ? h(View, { style: styles.totalRow(rtl) }, [
                    h(Text, {}, `${t("fields.vat-rate")} (${config.vat}%)`),
                    h(Text, {}, n(subtotal * (config.vat / 100), "currency")),
                  ])
                : null,

              h(View, { style: styles.totalFinal(rtl) }, [
                h(Text, {}, t("fields.total")),
                h(Text, {}, n(totalWithVat, "currency")),
              ]),

              h(View, { style: styles.totalFinal(rtl) }, [
                h(Text, {}, useTotalAsText().numberToText(totalWithVat, locale.value)),
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
