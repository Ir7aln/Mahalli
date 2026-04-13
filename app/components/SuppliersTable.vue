<script setup lang="ts">
import { FilePenLine, GripHorizontal, Trash2 } from "lucide-vue-next";
import { convertFileSrc } from "@tauri-apps/api/core";
import { SupplierDelete, SupplierUpdate } from "#components";

defineProps<{
  suppliers: SupplierT[];
}>();
const { t, locale } = useI18n();
const modal = useModal();

function toggleThisSupplier(supplier: SupplierT, name: "delete" | "update") {
  if (name === "delete") {
    modal.open(SupplierDelete, {
      id: supplier.id!,
      fullName: supplier.full_name,
    });
  } else {
    modal.open(SupplierUpdate, {
      id: supplier.id!,
      fullName: supplier.full_name,
      email: supplier.email,
      phoneNumber: supplier.phone_number,
      address: supplier.address,
    });
  }
}
</script>

<template>
  <div class="w-full pb-16">
    <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
      <TableHeader>
        <TableRow>
          <TableHead class="w-14" />
          <TableHead>{{ t("fields.full-name") }}</TableHead>
          <TableHead>{{ t("fields.email") }}</TableHead>
          <TableHead>{{ t("fields.phone") }}</TableHead>
          <TableHead>{{ t("fields.address") }}</TableHead>
          <TableHead class="w-20">
            {{ t("fields.actions") }}
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow
          v-for="(supplier, index) in suppliers"
          :key="supplier.id"
          v-fade="index"
        >
          <TableCell class="p-2 flex justify-center">
            <Avatar>
              <AvatarImage
                v-if="supplier.image"
                :src="convertFileSrc(supplier.image)"
              />
              <AvatarFallback class="text-xs">
                {{ supplier.full_name.substring(0, 5) }}
              </AvatarFallback>
            </Avatar>
          </TableCell>
          <TableCell class="p-2 whitespace-nowrap font-medium">
            {{ supplier?.full_name }}
          </TableCell>
          <TableCell class="p-2">
            {{ supplier.email || "--" }}
          </TableCell>
          <TableCell class="p-2">
            {{ supplier.phone_number || "--" }}
          </TableCell>
          <TableCell class="p-2">
            {{ supplier.address || "--" }}
          </TableCell>
          <TableCell class="p-2">
            <div class="flex justify-center">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem
                    @click="toggleThisSupplier(supplier, 'update')"
                  >
                    <FilePenLine
                      :size="20"
                      class="text-slate-800 inline mr-2"
                    />
                    {{ t("buttons.edit") }}
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    @click="toggleThisSupplier(supplier, 'delete')"
                  >
                    <Trash2 :size="20" class="text-red-500 inline mr-2" />
                    <span class="text-red-500">
                      {{ t("buttons.delete") }}
                    </span>
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
    <Pagination />
  </div>
</template>
