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
import { commands } from "@/bindings";

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

  const styles = {
    page: (rtl: boolean) => ({
      paddingTop: 28,
      paddingBottom: 28,
      paddingHorizontal: 28,
      fontSize: 10,
      color: "#111827",
      fontFamily: "Cairo",
      direction: (rtl ? "rtl" : "ltr") as "rtl" | "ltr",
      backgroundColor: "#ffffff",
    }),

    // Large title — matches text-5xl font-medium
    header: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      justifyContent: "space-between" as const,
      alignItems: "center" as const,
      marginBottom: 20,
      paddingBottom: 12,
      borderBottomWidth: 1,
      borderBottomColor: "#e5e7eb",
    }),

    title: (rtl: boolean) => ({
      fontSize: 40,
      fontWeight: 500,
      color: "#111827",
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    logo: {
      width: 90,
      height: "auto",
    },

    // Section headings — matches text-xl font-bold border-b-2 border-gray-200 pb-2 mb-4
    sectionTitle: (rtl: boolean) => ({
      fontSize: 13,
      fontWeight: 700,
      color: "#1f2937",
      borderBottomWidth: 2,
      borderBottomColor: "#e5e7eb",
      paddingBottom: 6,
      marginBottom: 10,
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    // Standard body text — matches text-gray-700
    text: (rtl: boolean) => ({
      fontSize: 10,
      color: "#374151",
      marginBottom: 3,
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    // Bold name line — matches font-bold text-gray-900
    bold: (rtl: boolean) => ({
      fontSize: 10,
      fontWeight: 700,
      color: "#111827",
      marginBottom: 3,
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    muted: (rtl: boolean) => ({
      fontSize: 9,
      color: "#6b7280",
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    // Two-column layout — matches grid grid-cols-2 gap-8
    grid: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      gap: 32,
      marginBottom: 20,
    }),

    col: {
      width: "50%",
    },

    // Table header row — matches bg-gray-200 border border-gray-500 p-3 font-semibold
    tableHeader: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      backgroundColor: "#e5e7eb",
      borderBottomWidth: 1,
      borderBottomColor: "#6b7280",
      paddingVertical: 8,
      paddingHorizontal: 6,
      fontSize: 10,
      fontWeight: 700,
      color: "#1f2937",
    }),

    // Table data rows — matches border border-gray-500 p-3
    tableRow: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      borderBottomWidth: 1,
      borderBottomColor: "#d1d5db",
      paddingVertical: 8,
      paddingHorizontal: 6,
      fontSize: 10,
      color: "#1f2937",
    }),

    cellName: (rtl: boolean) => ({
      width: "45%",
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    cellQty: {
      width: "15%",
      textAlign: "right" as const,
    },

    cellPrice: {
      width: "20%",
      textAlign: "right" as const,
    },

    cellTotal: {
      width: "20%",
      textAlign: "right" as const,
    },

    // Totals box — right-aligned, ~1/3 width
    totals: (rtl: boolean) => ({
      marginTop: 20,
      alignSelf: (rtl ? "flex-start" : "flex-end") as "flex-start" | "flex-end",
      width: "38%",
      marginBottom: 6,
    }),

    // Normal totals row — matches flex justify-between py-1 text-gray-700
    totalRow: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      justifyContent: "space-between" as const,
      paddingVertical: 4,
      fontSize: 10,
      color: "#374151",
    }),

    // Grand total row — matches text-xl font-bold border-t-2 border-gray-500 py-2 mt-2
    totalFinal: (rtl: boolean) => ({
      flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
      justifyContent: "space-between" as const,
      paddingVertical: 8,
      marginTop: 8,
      borderTopWidth: 2,
      borderTopColor: "#6b7280",
      fontSize: 13,
      fontWeight: 700,
      color: "#111827",
    }),

    // Total in words — full-width muted line below the totals box
    totalWords: (rtl: boolean) => ({
      marginTop: 6,
      fontSize: 9,
      color: "#6b7280",
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    // Seller details section at bottom
    sellerDetails: {
      marginTop: 24,
      marginBottom: 8,
    },

    // Inline "Label: value" row in seller details
    detailItem: (rtl: boolean) => ({
      fontSize: 10,
      color: "#1f2937",
      marginBottom: 4,
      textAlign: (rtl ? "right" : "left") as "right" | "left",
    }),

    // Footer bar — matches bg-gray-50 p-4 text-center text-sm text-gray-600 border-t
    footer: {
      marginTop: 12,
      paddingVertical: 10,
      paddingHorizontal: 16,
      backgroundColor: "#f9fafb",
      borderTopWidth: 1,
      borderTopColor: "#e5e7eb",
      textAlign: "center" as const,
    },

    bgTemplate: {
      position: "absolute" as const,
      left: 0,
      top: 0,
      width: "100%",
      height: "100%",
      zIndex: -1,
    },
  };

  // Inline bold label helper for seller detail rows
  function detailRow(rtl: boolean, label: string, value: string) {
    return h(Text, { style: styles.detailItem(rtl) }, () => [
      h(Text, { style: { fontWeight: 600, color: "#374151" } }, `${label}: `),
      value,
    ]);
  }

  async function generatePdf(data: any, type: DocType) {
    if (!data) return "";

    try {
      const rtl = isArabicLocale(locale.value);

      const templateImage = config.template.bytes
        ? toDataUri(config.template.bytes, config.template.name)
        : null;

      const sellerResult = await commands.getSellerProfile();
      const seller = sellerResult.status === "ok" ? sellerResult.data.data : null;

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

      const hasSellerInfo = !!(
        seller?.legal_name ||
        seller?.ice ||
        seller?.if_number ||
        seller?.rc ||
        seller?.patente
      );

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

            // HEADER
            h(View, { style: styles.header(rtl) }, () => [
              h(View, {}, () => [
                h(Text, { style: styles.title(rtl) }, type.toUpperCase()),
                h(Text, { style: styles.muted(rtl) }, safeText(data.identifier)),
                h(Text, { style: styles.muted(rtl) }, safeText(createdAtLabel)),
                config.fields.status
                  ? h(Text, { style: styles.muted(rtl) }, safeText(statusLabel))
                  : null,
              ]),

              templateImage
                ? h(Image, { src: templateImage, style: styles.logo })
                : null,
            ]),

            // CUSTOMER
            h(View, { style: { marginBottom: 20 } }, () => [
              h(Text, { style: styles.sectionTitle(rtl) }, t("fields.bill-to")),

              h(Text, { style: styles.bold(rtl) },
                safeText(data.client?.full_name ?? data.full_name)),

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

            // TABLE
            h(View, { style: { marginBottom: 20 } }, () => [
              h(View, { style: styles.tableHeader(rtl) }, () => [
                h(Text, { style: styles.cellName(rtl) }, t("fields.product")),
                h(Text, { style: styles.cellQty }, t("fields.quantity")),
                h(Text, { style: styles.cellPrice }, t("fields.price")),
                h(Text, { style: styles.cellTotal }, t("fields.total")),
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

            // TOTALS BOX — right-aligned, ~38% wide
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
            ]),

            // TOTAL IN WORDS — full-width muted line
            h(Text, { style: styles.totalWords(rtl) },
              useTotalAsText().numberToText(totalWithVat, locale.value)),

            // SELLER DETAILS (bottom)
            hasSellerInfo
              ? h(View, { style: styles.sellerDetails }, () => [
                  h(Text, { style: styles.sectionTitle(rtl) }, t("seller-profile.title")),

                  h(View, { style: styles.grid(rtl) }, () => [
                    // LEFT — name / contact
                    h(View, { style: styles.col }, () => [
                      seller?.legal_name
                        ? detailRow(rtl, t("seller-profile.fields.legal-name"), safeText(seller.legal_name))
                        : null,

                      seller?.commercial_name
                        ? detailRow(rtl, t("seller-profile.fields.commercial-name"), safeText(seller.commercial_name))
                        : null,

                      seller?.email
                        ? detailRow(rtl, t("fields.email"), safeText(seller.email))
                        : null,

                      seller?.phone_number
                        ? detailRow(rtl, t("fields.phone"), safeText(seller.phone_number))
                        : null,

                      (seller?.address || seller?.city)
                        ? detailRow(rtl, t("fields.address"),
                            [seller?.address, seller?.city].filter(Boolean).join(", "))
                        : null,
                    ]),

                    // RIGHT — legal identifiers
                    h(View, { style: styles.col }, () => [
                      seller?.ice
                        ? detailRow(rtl, t("fields.ice"), safeText(seller.ice))
                        : null,

                      seller?.if_number
                        ? detailRow(rtl, t("fields.if-number"), safeText(seller.if_number))
                        : null,

                      seller?.rc
                        ? detailRow(rtl, t("fields.rc"), safeText(seller.rc))
                        : null,

                      seller?.patente
                        ? detailRow(rtl, t("fields.patente"), safeText(seller.patente))
                        : null,
                    ]),
                  ]),
                ])
              : null,

            // FOOTER — gray bar, centered, matches bg-gray-50 text-center border-t
            seller?.invoice_footer
              ? h(View, { style: styles.footer }, () => [
                  h(Text, { style: { fontSize: 9, color: "#4b5563" } },
                    safeText(seller.invoice_footer)),
                ])
              : null,
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
