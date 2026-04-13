import type { ORDER_STATUSES } from "~/consts";

export interface OrderT {
  id: string;
  status: (typeof ORDER_STATUSES)[number];
  client_id: string;
  full_name?: string;
  products?: number;
  total?: number;
  created_at?: string;
  identifier: string;
}

export interface OrderForUpdateT {
  id: string;
  client_id: string;
  full_name: string;
  created_at: string;
  identifier?: string;
  status: (typeof ORDER_STATUSES)[number];
  items: {
    id: string;
    inventory_id: string;
    name: string;
    product_id: string;
    quantity: number;
    price: number;
  }[];
}

export interface OrderProductsPreviewT {
  price: number;
  name: string;
  quantity: number;
}
