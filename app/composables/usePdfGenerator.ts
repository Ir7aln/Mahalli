import { PDFDocument, PDFName, PDFPage, PageSizes, rgb } from "pdf-lib";
import type { PDFFont } from "pdf-lib";
import fontkit from "@pdf-lib/fontkit";
import { toast } from "vue-sonner";
import * as Logger from "@tauri-apps/plugin-log";
import CairoRegular from "@/assets/fonts/Cairo-Regular.ttf";

export function usePdfGenerator() {
  const { t, locale, n, d } = useI18n();
  const { numberToText } = useNumberToText();

  const Width = ref(0);
  const Height = ref(0);

  let pdfDoc: PDFDocument;
  let font: PDFFont;
  let page: PDFPage;
  let template: PDFPage | undefined;

  const config = reactive({
    marginTop: 40,
    marginX: 20,
    marginBottom: 40,
    vat: 20,
    template: {
      bytes: null as Uint8Array | null,
      name: null as string | null,
    },
    mainColor: rgb(0, 0, 0),
    secondaryColor: rgb(0.25, 0.25, 0.25),
    fields: {
      full_name: true,
      email: true,
      address: true,
      phone_number: true,
      status: true,
      vat: true,
    },
  });

  async function initPdfDoc() {
    try {
      pdfDoc = await PDFDocument.create();
      pdfDoc.registerFontkit(fontkit);
      const res: any = await $fetch(CairoRegular);
      const fontBytes = await res.arrayBuffer();
      font = await pdfDoc.embedFont(fontBytes);
    } catch (err: any) {
      handleError(err);
    }
  }

  type DocType = "order" | "invoice" | "quote";

  async function generatePdf(data: any, type: DocType) {
    await initPdfDoc();
    if (!pdfDoc || !font) 
      return;

    setPdfMetadata(pdfDoc, data, type);

    try {
      await setupPage();
      drawContent(data, type);
      return await pdfDoc.saveAsBase64({ dataUri: true });
    } catch (err) {
      handleError(err);
    }
  }

  function setPdfMetadata(pdfDoc: PDFDocument, data: any, type: DocType) {
    pdfDoc.setTitle(data.identifier);
    pdfDoc.setAuthor("trymahalli.com");
    pdfDoc.setSubject(type);
    pdfDoc.setProducer("trymahalli.com");
    pdfDoc.setCreator("trymahalli.com");
  }

  async function setupPage() {
    if (!pdfDoc) 
      return;

    if (config.template.bytes) {
      const fileName = config.template.name?.toLowerCase() || "";
      if (fileName.endsWith(".pdf")) {
        const sourcePdfDoc = await PDFDocument.load(config.template.bytes);
        const [templatePage] = await pdfDoc.copyPages(sourcePdfDoc, [0]);
        template = templatePage;
        page = pdfDoc.addPage(copyPage(template));
      } else {
        // Add empty page first
        page = pdfDoc.addPage();
        page.setSize(...PageSizes.A4);

        let image;

        if (fileName.endsWith(".jpg") || fileName.endsWith(".jpeg")) {
          image = await pdfDoc.embedJpg(config.template.bytes);
        } else if (fileName.endsWith(".png")) {
          image = await pdfDoc.embedPng(config.template.bytes);
        } else {
          // Default to PNG if can't determine
          try {
            image = await pdfDoc.embedPng(config.template.bytes);
          } catch {
            try {
              image = await pdfDoc.embedJpg(config.template.bytes);
            } catch {
              handleError(new Error("Unsupported image format"));
              return;
            }
          }
        }

        // Draw the image as background
        const { width, height } = page.getSize();
        const dims = image.scale(1);

        // Scale to fit page while maintaining aspect ratio
        const scale = Math.min(width / dims.width, height / dims.height);

        // Center the image on the page
        const x = (width - dims.width * scale) / 2;
        const y = (height - dims.height * scale) / 2;

        page.drawImage(image, {
          x,
          y,
          width: dims.width * scale,
          height: dims.height * scale,
        });
      }
    } else {
      page = pdfDoc.addPage();
    }

    if (!page) {
      page = pdfDoc.addPage();
    }

    page.setSize(...PageSizes.A4);
    const { width, height } = page.getSize();
    Width.value = width;
    Height.value = height;
  }

  function drawContent(data: any, type: DocType) {
    if (!page || !font) 
      return;
    const vat = config.fields.vat ? config.vat : 0;
    const vatTotal = data.total * (vat / 100);
    Height.value -= config.marginTop;
    drawHeader(data, type);
    drawSenderAndReceiver(data.client);
    drawTableHeaders();
    drawItems(data.items);
    drawSummary(getSummaryItems(data.total, vatTotal));
    drawTotalAsText(data.total + vatTotal);
  }

  function drawHeader(data: any, type: DocType) {
    if (!page || !font) 
      return;

    const headerText = capitalizeFirstLetter(t(`fields.${type}`));
    const totalText = n(
      data.total + data.total * (config.vat / 100),
      "currency",
    );

    page.drawText(headerText, {
      x: config.marginX - 1,
      y: Height.value,
      font,
      size: 30,
      color: config.mainColor,
    });

    page.drawText(reverseText(totalText), {
      x: Width.value - config.marginX - getTextWidth(totalText, 20),
      y: Height.value,
      font,
      size: 20,
      color: config.mainColor,
    });

    page.drawText(data.identifier, {
      x: config.marginX,
      y: Height.value - 20,
      font,
      size: 13,
      color: config.secondaryColor,
    });

    if (type !== "quote" && config.fields.status) {
      const statusText = t(`status.${data.status.toLowerCase()}`);
      page.drawText(statusText, {
        x: getMiddleX(statusText, Width.value, 13),
        y: Height.value - 20,
        font,
        size: 13,
        color: config.secondaryColor,
      });
    }

    const dateText = d(new Date(data.created_at));
    page.drawText(dateText, {
      x: Width.value - config.marginX - getTextWidth(dateText, 13),
      y: Height.value - 20,
      font,
      size: 13,
      color: config.secondaryColor,
    });

    Height.value -= 50;
  }

  function drawSenderAndReceiver(client: any) {
    if (!page || !font) 
      return;

    page.drawText(t("fields.bill-to").toUpperCase(), {
      x: config.marginX,
      y: Height.value,
      font,
      size: 14,
      color: config.mainColor,
    });

    const fields = getClientFields(client);
    let index = 0;
    for (const field of fields) {
      page.drawText(field, {
        x: config.marginX,
        y: Height.value - 20 * (index + 1),
        font,
        size: 13,
        color: config.secondaryColor,
      });
      index++;
    }

    Height.value -= 20 * fields.length + 20;
  }

  function getClientFields(client: any): string[] {
    return ["full_name", "email", "address", "phone_number"]
      .filter(value => config.fields[value])
      .map(key => client[key])
      .filter(value => value !== null);
  }

  function drawTableHeaders() {
    if (!page || !font) 
      return;

    drawHorizontalLine(Height.value);

    const headers = ["name", "quantity", "price", "total"];
    let index = 0;
    for (const header of headers) {
      page.drawText(t(`fields.${header}`), {
        x: config.marginX + 5 + (Width.value * index) / 4,
        y: Height.value - 20,
        font,
        size: 14,
        color: config.mainColor,
      });
      index++;
    }

    drawHorizontalLine(Height.value - 30);

    Height.value -= 40;
  }

  function drawItems(items: Record<string, string | number>[]) {
    for (const item of items) {
      if (Height.value < config.marginBottom) {
        addNewPage();
      }

      if (!page || !font) 
        return;

      page.drawText(item.name, {
        x: config.marginX + 5,
        y: Height.value - 10,
        font,
        size: 12,
        color: config.secondaryColor,
      });

      page.drawText(n(item?.quantity, "decimal"), {
        x: config.marginX + 5 + Width.value / 4,
        y: Height.value - 10,
        font,
        size: 12,
        color: config.secondaryColor,
      });

      page.drawText(reverseText(n(item.price, "currency")), {
        x: config.marginX + 5 + Width.value / 2,
        y: Height.value - 10,
        font,
        size: 12,
        color: config.secondaryColor,
      });

      page.drawText(reverseText(n(item.price * item.quantity, "currency")), {
        x: config.marginX + 5 + (Width.value * 3) / 4,
        y: Height.value - 10,
        font,
        size: 12,
        color: config.secondaryColor,
      });

      drawHorizontalLine(Height.value - 20, config.marginX, 0.3);

      Height.value -= 30;
    }
  }

  function drawSummary(summaryItems: Record<string, string | number>[]) {
    for (const item of summaryItems) {
      if (Height.value < config.marginBottom) {
        addNewPage();
      }

      if (!page || !font) 
        return;

      const summaryX = getSummaryX(Width.value);

      page.drawText(t(`fields.${item.label}`).toUpperCase(), {
        x: summaryX,
        y: Height.value - 10,
        font,
        size: 12,
        color: config.mainColor,
      });

      page.drawText(item.value, {
        x: config.marginX + 5 + (Width.value * 3) / 4,
        y: Height.value - 10,
        font,
        size: 12,
        color: config.secondaryColor,
      });

      drawHorizontalLine(Height.value - 20, summaryX, 0.3);

      Height.value -= 30;
    }
  }

  function drawTotalAsText(total: number) {
    if (!page || !font) 
      return;

    const totalAsText = numberToText(total, locale.value as any);

    page.drawText(totalAsText, {
      x: getMiddleX(totalAsText, Width.value, 13),
      y: Height.value - 30,
      font,
      size: 13,
      color: config.mainColor,
    });
  }

  function copyPage(originalPage: PDFPage) {
    if (!pdfDoc) 
      return originalPage;

    const cloneNode = originalPage.node.clone();
    const { Contents } = originalPage.node.normalizedEntries();
    if (Contents) 
      cloneNode.set(PDFName.of("Contents"), Contents.clone());
    const cloneRef = pdfDoc.context.register(cloneNode);
    return PDFPage.of(cloneNode, cloneRef, pdfDoc);
  }

  function getTextWidth(text: string, fontSize: number) {
    return font?.widthOfTextAtSize(text, fontSize) || 0;
  }

  function getMiddleX(text: string, width: number, fontSize: number) {
    return (width - getTextWidth(text, fontSize)) / 2;
  }

  function capitalizeFirstLetter(text: string) {
    return text.charAt(0).toUpperCase() + text.slice(1);
  }

  function reverseText(text: string) {
    if (locale.value !== "ar") 
      return text;
    const currency = text.split("").splice(-5).join("");
    const amount = text
      .split("")
      .splice(0, text.split("").length - 6)
      .reverse()
      .join("");
    return `${amount} ${currency}`;
  }

  function getSummaryX(width: number) {
    const offsets = { en: 60, fr: 60, de: -20, ar: 30 };
    return (
      width - width / 2 + (offsets[locale.value as keyof typeof offsets] || 60)
    );
  }

  function getSummaryItems(total: number, vatTotal: number) {
    if (!config.fields.vat) {
      return [
        {
          label: "total",
          value: reverseText(n(total, "currency")),
        },
      ];
    }
    return [
      { label: "sub-total", value: reverseText(n(total, "currency")) },
      { label: "vat-rate", value: `${config.vat}%` },
      {
        label: "vat-amount",
        value: reverseText(n(vatTotal, "currency")),
      },
      {
        label: "grand-total",
        value: reverseText(n(total + vatTotal, "currency")),
      },
    ];
  }

  function addNewPage() {
    if (!pdfDoc) 
      return;

    let newPage: PDFPage;
    if (template) {
      newPage = pdfDoc.addPage(copyPage(template));
    } else {
      newPage = pdfDoc.addPage();
    }
    newPage.setSize(...PageSizes.A4);
    page = newPage;
    const { width, height } = newPage.getSize();
    Width.value = width;
    Height.value = height - config.marginTop;
  }

  function drawHorizontalLine(y: number, startX = config.marginX, o = 0.75) {
    page?.drawLine({
      start: { x: startX, y },
      end: { x: Width.value - config.marginX, y },
      thickness: 1,
      color: config.mainColor,
      opacity: o,
    });
  }

  function handleError(err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR PDF-LIB: ${err}`);
  }

  return {
    config,
    generatePdf,
  };
}
