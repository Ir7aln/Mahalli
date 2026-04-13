export interface ProductT {
  id?: string;
  name: string;
  selling_price: number;
  purchase_price: number;
  image?: string;
  description?: string;
  min_quantity: number;
  inventory?: number;
  created_at?: number;
}
