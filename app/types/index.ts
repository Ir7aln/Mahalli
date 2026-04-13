import type { LocationQueryValue } from "vue-router";

export interface Res<T> {
  error?: string;
  message?: string;
  data: T;
}

export interface QueryParams {
  search: LocationQueryValue | LocationQueryValue[];
  page: number | LocationQueryValue | LocationQueryValue[];
  status?: LocationQueryValue | LocationQueryValue[];
  limit: number | LocationQueryValue | LocationQueryValue[];
  transaction_type?: LocationQueryValue | LocationQueryValue[];
  created_at?: LocationQueryValue | LocationQueryValue[];
}
