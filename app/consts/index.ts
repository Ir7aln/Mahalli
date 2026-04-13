export const CLIENT_FIELDS = [
  {
    label: "full-name",
    field: "full_name",
  },
  {
    label: "email",
    field: "email",
  },
  {
    label: "phone",
    field: "phone_number",
  },
  {
    label: "address",
    field: "address",
  },
];

export const STATUS_COLORS = {
  DRAFT: "bg-gray-100 border-gray-500 text-gray-900",
  SENT: "bg-blue-100 border-blue-500 text-blue-900",
  PAID: "bg-green-100 border-green-500 text-green-900",
  PARTIALLY_PAID: "bg-teal-100 border-teal-500 text-teal-900",
  OVERDUE: "bg-orange-100 border-orange-500 text-orange-900",
  CANCELLED: "bg-red-100 border-red-500 text-red-900",
  PENDING: "bg-yellow-100 border-yellow-500 text-yellow-900",
  PROCESSING: "bg-blue-100 border-blue-500 text-blue-900",
  SHIPPED: "bg-indigo-100 border-indigo-500 text-indigo-900",
  DELIVERED: "bg-green-100 border-green-500 text-green-900",
} as const;

export const INVOICE_STATUSES = [
  "DRAFT",
  "SENT",
  "PAID",
  "PARTIALLY_PAID",
  "OVERDUE",
  "CANCELLED",
] as const;

export const ORDER_STATUSES = [
  "PENDING",
  "PROCESSING",
  "SHIPPED",
  "DELIVERED",
  "CANCELLED",
] as const;
