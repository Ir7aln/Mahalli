import type { INVOICE_STATUSES } from "~/consts";

export interface InvoiceT {
  id: string;
  status: (typeof INVOICE_STATUSES)[number];
  paid_amount: number;
  client_id: string;
  order_id: string;
  full_name?: string;
  products?: number;
  total?: number;
  created_at?: string;
  identifier: string;
}

export interface InvoiceForUpdateT {
  id: string;
  client_id: string;
  full_name: string;
  paid_amount: number;
  created_at: string;
  status: (typeof INVOICE_STATUSES)[number];
  items: {
    id: string;
    inventory_id: string;
    name: string;
    product_id: string;
    quantity: number;
    price: number;
  }[];
}

export interface InvoiceProductsPreviewT {
  price: number;
  name: string;
  quantity: number;
}
