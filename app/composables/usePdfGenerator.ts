import { computed, h, reactive, ref } from "vue";
import { Document, Page, Text, View, usePdf, fontStore } from "@ceereals/vue-pdf";
import * as Logger from "@tauri-apps/plugin-log";
import { commands } from "@/bindings";

type DocType = "order" | "invoice" | "quote" | "delivery-note" | "credit-note";

let fontsRegistered = false;

function registerPdfFonts() {
  if (fontsRegistered) return;
  fontStore.register({ family: "Cairo", src: "/fonts/Cairo-Regular.ttf" });
  fontsRegistered = true;
}

function isArabicLocale(locale: string) {
  return ["ar", "ar-MA", "ar-SA", "ar-AE", "ar-EG"].some((code) => locale.startsWith(code));
}

function safeText(value: unknown) {
  return String(value ?? "-");
}

export function usePdfGenerator() {
  const { t, n, d, locale } = useI18n();
  const { showErrorToast } = useCommandError();

  registerPdfFonts();

  const isGenerating = ref(false);

  let sellerCache: { data: any } | null = null;

  async function getSeller() {
    if (sellerCache) return sellerCache.data;
    const result = await commands.getSellerProfile();
    sellerCache = { data: result.status === "ok" ? result.data.data : null };
    return sellerCache.data;
  }

  const config = reactive({
    vat: 20,
    fields: {
      vat: true,
      full_name: true,
      email: true,
      phone_number: true,
      address: true,
    },
  });

  async function generatePdf(data: any, type: DocType, scale = 1) {
    if (!data) return "";

    isGenerating.value = true;

    const s = (v: number) => v * scale;

    const styles = {
      page: (rtl: boolean) => ({
        paddingTop: s(28),
        paddingBottom: s(28),
        paddingHorizontal: s(28),
        fontSize: s(10),
        color: "#111827",
        fontFamily: "Cairo",
        direction: (rtl ? "rtl" : "ltr") as "rtl" | "ltr",
        backgroundColor: "#ffffff",
      }),

      header: (rtl: boolean) => ({
        flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
        justifyContent: "space-between" as const,
        alignItems: "center" as const,
        marginBottom: s(20),
        paddingBottom: s(12),
        borderBottomWidth: 1,
        borderBottomColor: "#e5e7eb",
      }),

      title: (rtl: boolean) => ({
        fontSize: s(40),
        fontWeight: 500,
        color: "#111827",
        textAlign: (rtl ? "right" : "left") as "right" | "left",
      }),

      sectionTitle: (rtl: boolean) => ({
        fontSize: s(13),
        fontWeight: 700,
        color: "#1f2937",
        borderBottomWidth: 2,
        borderBottomColor: "#e5e7eb",
        paddingBottom: s(6),
        marginBottom: s(10),
        textAlign: (rtl ? "right" : "left") as "right" | "left",
      }),

      text: (rtl: boolean) => ({
        fontSize: s(10),
        color: "#374151",
        marginBottom: s(3),
        textAlign: (rtl ? "right" : "left") as "right" | "left",
      }),

      bold: (rtl: boolean) => ({
        fontSize: s(10),
        fontWeight: 700,
        color: "#111827",
        marginBottom: s(3),
        textAlign: (rtl ? "right" : "left") as "right" | "left",
      }),

      muted: (rtl: boolean) => ({
        fontSize: s(9),
        color: "#6b7280",
        textAlign: (rtl ? "right" : "left") as "right" | "left",
      }),

      grid: (rtl: boolean) => ({
        flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
        gap: s(32),
        marginBottom: s(20),
      }),

      col: { width: "50%" },

      tableHeader: (rtl: boolean) => ({
        flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
        backgroundColor: "#e5e7eb",
        borderBottomWidth: 1,
        borderBottomColor: "#6b7280",
        paddingVertical: s(8),
        paddingHorizontal: s(6),
        fontSize: s(10),
        fontWeight: 700,
        color: "#1f2937",
      }),

      tableRow: (rtl: boolean) => ({
        flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
        borderBottomWidth: 1,
        borderBottomColor: "#d1d5db",
        paddingVertical: s(8),
        paddingHorizontal: s(6),
        fontSize: s(10),
        color: "#1f2937",
      }),

      cellName: (rtl: boolean) => ({
        width: "45%",
        textAlign: (rtl ? "right" : "left") as "right" | "left",
      }),

      cellQty: { width: "15%", textAlign: "right" as const },
      cellPrice: { width: "20%", textAlign: "right" as const },
      cellTotal: { width: "20%", textAlign: "right" as const },

      totals: (rtl: boolean) => ({
        marginTop: s(20),
        alignSelf: (rtl ? "flex-start" : "flex-end") as "flex-start" | "flex-end",
        width: "38%",
        marginBottom: s(6),
      }),

      totalRow: (rtl: boolean) => ({
        flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
        justifyContent: "space-between" as const,
        paddingVertical: s(4),
        fontSize: s(10),
        color: "#374151",
      }),

      totalFinal: (rtl: boolean) => ({
        flexDirection: (rtl ? "row-reverse" : "row") as "row-reverse" | "row",
        justifyContent: "space-between" as const,
        paddingVertical: s(8),
        marginTop: s(8),
        borderTopWidth: 2,
        borderTopColor: "#6b7280",
        fontSize: s(13),
        fontWeight: 700,
        color: "#111827",
      }),

      totalWords: (rtl: boolean) => ({
        marginTop: s(6),
        fontSize: s(9),
        color: "#6b7280",
        textAlign: (rtl ? "right" : "left") as "right" | "left",
      }),

      sellerDetails: {
        marginTop: s(24),
        marginBottom: s(8),
      },

      detailItem: (rtl: boolean) => ({
        fontSize: s(10),
        color: "#1f2937",
        marginBottom: s(4),
        textAlign: (rtl ? "right" : "left") as "right" | "left",
      }),

      footer: {
        marginTop: s(12),
        paddingVertical: s(10),
        paddingHorizontal: s(16),
        backgroundColor: "#f9fafb",
        borderTopWidth: 1,
        borderTopColor: "#e5e7eb",
        textAlign: "center" as const,
      },
    };

    function detailRow(rtl: boolean, label: string, value: string) {
      return h(Text, { style: styles.detailItem(rtl) }, () => [
        h(Text, { style: { fontWeight: 600, color: "#374151" } }, `${label}: `),
        value,
      ]);
    }

    try {
      const rtl = isArabicLocale(locale.value);
      const seller = await getSeller();

      const items = Array.isArray(data.items) ? data.items : [];
      const vatMultiplier = 1 + (Number(config.vat) || 0) / 100;
      const subtotal = Number(data.total ?? 0);
      const totalWithVat = config.fields.vat ? subtotal * vatMultiplier : subtotal;

      const statusLabel = data.status ? t(`status.${String(data.status).toLowerCase()}`) : null;
      const createdAtLabel = data.created_at ? d(new Date(data.created_at), "long") : "-";

      const hasSellerInfo = !!(
        seller?.legal_name ||
        seller?.ice ||
        seller?.if_number ||
        seller?.rc ||
        seller?.patente
      );

      const clientEmail = data.client?.email ?? data.email;
      const clientPhone = data.client?.phone_number ?? data.phone_number;
      const clientAddress = data.client?.address ?? data.address;

      const doc = h(Document, { title: data.identifier ?? type, author: "Mahalli" }, () => [
        h(Page, { size: "A4", style: styles.page(rtl) }, () => [

          // HEADER
          h(View, { style: styles.header(rtl) }, () => [
            h(View, {}, () => [
              h(Text, { style: styles.title(rtl) }, type.toUpperCase()),
              h(Text, { style: styles.muted(rtl) }, safeText(data.identifier)),
              h(Text, { style: styles.muted(rtl) }, safeText(createdAtLabel)),
              statusLabel ? h(Text, { style: styles.muted(rtl) }, statusLabel) : null,
            ]),
          ]),

          // CUSTOMER
          h(View, { style: { marginBottom: s(20) } }, () => [
            h(Text, { style: styles.sectionTitle(rtl) }, t("fields.bill-to")),
            config.fields.full_name
              ? h(Text, { style: styles.bold(rtl) }, safeText(data.client?.full_name ?? data.full_name))
              : null,
            config.fields.email && clientEmail
              ? h(Text, { style: styles.text(rtl) }, safeText(clientEmail))
              : null,
            config.fields.phone_number && clientPhone
              ? h(Text, { style: styles.text(rtl) }, safeText(clientPhone))
              : null,
            config.fields.address && clientAddress
              ? h(Text, { style: styles.text(rtl) }, safeText(clientAddress))
              : null,
          ]),

          // TABLE
          h(View, { style: { marginBottom: s(20) } }, () => [
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

          // TOTALS
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

          // TOTAL IN WORDS
          h(Text, { style: styles.totalWords(rtl) },
            useTotalAsText().numberToText(totalWithVat, locale.value),
          ),

          // SELLER DETAILS
          hasSellerInfo
            ? h(View, { style: styles.sellerDetails }, () => [
                h(Text, { style: styles.sectionTitle(rtl) }, t("seller-profile.title")),
                h(View, { style: styles.grid(rtl) }, () => [
                  h(View, { style: styles.col }, () => [
                    seller?.legal_name ? detailRow(rtl, t("seller-profile.fields.legal-name"), safeText(seller.legal_name)) : null,
                    seller?.commercial_name ? detailRow(rtl, t("seller-profile.fields.commercial-name"), safeText(seller.commercial_name)) : null,
                    seller?.email ? detailRow(rtl, t("fields.email"), safeText(seller.email)) : null,
                    seller?.phone_number ? detailRow(rtl, t("fields.phone"), safeText(seller.phone_number)) : null,
                    seller?.address || seller?.city
                      ? detailRow(rtl, t("fields.address"), [seller?.address, seller?.city].filter(Boolean).join(", "))
                      : null,
                  ]),
                  h(View, { style: styles.col }, () => [
                    seller?.ice ? detailRow(rtl, t("fields.ice"), safeText(seller.ice)) : null,
                    seller?.if_number ? detailRow(rtl, t("fields.if-number"), safeText(seller.if_number)) : null,
                    seller?.rc ? detailRow(rtl, t("fields.rc"), safeText(seller.rc)) : null,
                    seller?.patente ? detailRow(rtl, t("fields.patente"), safeText(seller.patente)) : null,
                  ]),
                ]),
              ])
            : null,

          // FOOTER
          seller?.invoice_footer
            ? h(View, { style: styles.footer }, () => [
                h(Text, { style: { fontSize: s(9), color: "#4b5563" } }, safeText(seller.invoice_footer)),
              ])
            : null,
        ]),
      ]);

      const pdf = await usePdf(computed(() => doc), { reactive: false });
      await pdf.execute(true);
      return pdf.url.value ?? "";
    } catch (err: any) {
      showErrorToast(err);
      Logger.error(`PDF GENERATION ERROR: ${err?.message ?? JSON.stringify(err)}`);
      return "";
    } finally {
      isGenerating.value = false;
    }
  }

  return { config, generatePdf, isGenerating };
}
