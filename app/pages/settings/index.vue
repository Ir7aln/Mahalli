<script setup lang="ts">
const { t } = useI18n();
</script>

<template>
  <main class="w-full h-full">
    <div class="flex h-full flex-col gap-4">
      <div class="flex items-start justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-900">
            {{ t("settings.title") }}
          </h1>
          <p class="mt-1 text-slate-500">
            {{ t("settings.description") }}
          </p>
        </div>
      </div>

      <Tabs default-value="databases" class="w-full">
        <TabsList>
          <TabsTrigger value="databases">
            {{ t("settings.tabs.databases") }}
          </TabsTrigger>
          <TabsTrigger value="seller-profile">
            {{ t("settings.tabs.seller-profile") }}
          </TabsTrigger>
        </TabsList>

        <TabsContent value="databases">
          <SettingsDatabases />
        </TabsContent>

        <TabsContent value="seller-profile">
          <SettingsSellerProfile />
        </TabsContent>
      </Tabs>
    </div>
  </main>
</template>
