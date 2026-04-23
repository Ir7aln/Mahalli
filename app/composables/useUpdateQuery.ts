import { useRouter } from "vue-router";

export function useUpdateRouteQueryParams() {
  const router = useRouter();

  const updateQueryParams = (
    query: Record<string, string | number | boolean | null | undefined>,
  ) => {
    const route = router.currentRoute.value;
    const nextQuery = { ...route.query } as Record<string, string | number>;

    for (const [key, value] of Object.entries(query)) {
      if (value == null || value === "") {
        delete nextQuery[key];
        continue;
      }

      nextQuery[key] = typeof value === "boolean" ? String(value) : value;
    }

    router.push({
      path: route.path,
      query: nextQuery,
    });
  };

  return {
    updateQueryParams,
  };
}
