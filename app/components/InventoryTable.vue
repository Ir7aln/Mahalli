<script setup lang="ts">
defineProps<{
  inventory: InventoryT[];
}>();
const { t, d, locale, n } = useI18n();
</script>

<template>
  <div class="w-full pb-16">
    <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
      <TableHeader>
        <TableRow>
          <TableHead>{{ t("fields.name") }}</TableHead>
          <TableHead>{{ t("fields.price") }}</TableHead>
          <TableHead>{{ t("fields.quantity") }}</TableHead>
          <TableHead>{{ t("fields.status") }}</TableHead>
          <TableHead class="w-56">
            {{ t("fields.date") }}
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="(tx, index) in inventory" :key="tx.id" v-fade="index">
          <TableCell class="p-2 font-medium">
            {{ tx?.name }}
          </TableCell>
          <TableCell class="p-2">
            {{ n(tx?.price, "currency") }}
          </TableCell>
          <TableCell class="p-2">
            {{ `${tx.quantity} ${t("plrz.i", { n: Math.ceil(tx.quantity) })}` }}
          </TableCell>
          <TableCell class="p-2">
            <Badge
              variant="outline"
              :class="
                cn(
                  'cursor-pointer whitespace-nowrap',
                  tx?.transaction_type === 'OUT'
                    ? 'bg-green-100 border-green-500 text-green-900'
                    : 'bg-sky-100 border-sky-500 text-sky-900',
                )
              "
            >
              {{ t(`status.${tx?.transaction_type.toLowerCase()}`) }}
            </Badge>
          </TableCell>
          <TableCell class="p-2">
            {{ d(new Date(tx.created_at), "long") }}
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
    <Pagination />
  </div>
</template>
