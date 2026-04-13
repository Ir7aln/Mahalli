export interface InventoryT {
  id: string;
  name: string;
  transaction_type: "IN" | "OUT";
  quantity: number;
  price: number;
  created_at: string;
}
