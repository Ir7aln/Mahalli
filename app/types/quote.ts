export interface QuoteT {
  id: string;
  client_id: string;
  full_name?: string;
  products?: number;
  total?: number;
  created_at?: string;
  identifier: string;
}

export interface QuoteForUpdateT {
  id: string;
  client_id: string;
  full_name: string;
  created_at: string;
  items: {
    id?: string;
    name?: string;
    product_id: string;
    quantity: number;
    price: number;
  }[];
}

export interface QuoteProductsPreviewT {
  price: number;
  name: string;
  quantity: number;
}
