<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import { CLIENT_FIELDS, INVOICE_STATUSES, ORDER_STATUSES } from "~/consts";

const props = defineProps<{
  statues?: typeof INVOICE_STATUSES | typeof ORDER_STATUSES;
  documentType: "invoice" | "order" | "quote";
  config: any;
  document: any;
}>();
const emits = defineEmits<{
  updateConfig: [payload: ConfigSchemaT];
  saveConfig: [];
}>();
const { t } = useI18n();
const ConfigSchema = z.object({
  template: z.string().nullable(),
  marginTop: z.number().min(0),
  marginBottom: z.number().min(0),
  vat: z.number().min(0).max(100),
  fields: z.object({
    status: z.boolean(),
    full_name: z.boolean(),
    email: z.boolean(),
    phone_number: z.boolean(),
    address: z.boolean(),
    vat: z.boolean(),
  }),
  documentValues: z.object({
    status: z.enum([...INVOICE_STATUSES, ...ORDER_STATUSES]).optional(),
    client: z.object({
      full_name: z.string().optional(),
      email: z.string().email().optional(),
      phone_number: z.string().optional(),
      address: z.string().optional(),
    }),
  }),
});

type ConfigSchemaT = z.infer<typeof ConfigSchema>;

const configSchema = toTypedSchema(ConfigSchema);

const { handleSubmit, values, setFieldValue } = useForm<ConfigSchemaT>({
  validationSchema: configSchema,
  initialValues: {
    ...props.config,
    template: null,
    documentValues: {
      status: props.document.status,
      client: props.document.client,
    },
  },
});

function handleFileUpload(path: string) {
  setFieldValue("template", path);
}

function clearFileBytes() {
  setFieldValue("template", null);
}

const onSubmit = handleSubmit(async (values) => {
  emits("updateConfig", values);
});
</script>

<template>
  <form class="w-1/2 md:w-1/3 min-w-[500px]" @submit="onSubmit">
    <Card class="w-full h-full flex flex-col">
      <CardHeader>
        <CardTitle>{{ t("fields.configuration") }}</CardTitle>
      </CardHeader>
      <CardContent>
        <FormField name="template">
          <FormItem>
            <FormLabel>{{ t("fields.template") }}</FormLabel>
            <FormControl>
              <UiUploader
                name="Pdf"
                :extensions="['pdf', 'jpeg', 'png', 'jpg']"
                @save-path="handleFileUpload"
                @clear="clearFileBytes"
              />
            </FormControl>
          </FormItem>
        </FormField>

        <div class="grid grid-cols-2 gap-4">
          <FormField v-slot="{ componentField }" class="w-full" name="marginTop">
            <FormItem>
              <FormLabel>{{ t("fields.top-margin") }}</FormLabel>
              <FormControl>
                <Input v-bind="componentField" type="number" />
              </FormControl>
            </FormItem>
          </FormField>

          <FormField v-slot="{ componentField }" class="w-full" name="marginBottom">
            <FormItem>
              <FormLabel>{{ t("fields.bottom-margin") }}</FormLabel>
              <FormControl>
                <Input v-bind="componentField" type="number" />
              </FormControl>
            </FormItem>
          </FormField>
        </div>

        <Separator />

        <div v-if="documentType !== 'quote'" class="space-y-2">
          <FormField v-slot="{ value, handleChange }" name="fields.status">
            <FormItem class="flex justify-between items-end space-y-0">
              <FormLabel>{{ t("fields.status") }}</FormLabel>
              <FormControl>
                <Switch :checked="value" @update:checked="handleChange" />
              </FormControl>
            </FormItem>
          </FormField>

          <FormField v-slot="{ componentField }" name="documentValues.status">
            <FormItem>
              <Select :disabled="!values.fields.status" v-bind="componentField">
                <SelectTrigger>
                  <SelectValue
                    class="text-muted-foreground"
                    :placeholder="t('select-status')"
                  />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem
                      v-for="status in statues"
                      :key="status"
                      :value="status"
                    >
                      {{ t(`status.${status.toLowerCase()}`) }}
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </FormItem>
          </FormField>
        </div>

        <div class="space-y-2">
          <FormField v-slot="{ value, handleChange }" name="fields.vat">
            <FormItem class="flex justify-between items-end space-y-0">
              <FormLabel>{{ t("fields.vat-rate") }}</FormLabel>
              <FormControl>
                <Switch :checked="value" @update:checked="handleChange" />
              </FormControl>
            </FormItem>
          </FormField>

          <FormField v-slot="{ componentField }" name="vat">
            <FormItem>
              <FormControl>
                <Input
                  v-bind="componentField"
                  :disabled="!values.fields.vat"
                  type="number"
                >
                  <template #unite>
                    0% - 100%
                  </template>
                </Input>
              </FormControl>
            </FormItem>
          </FormField>
        </div>

        <Separator />

        <div v-for="item in CLIENT_FIELDS" :key="item.field" class="space-y-2">
          <FormField
            v-slot="{ value, handleChange }"
            :name="`fields.${item.field}`"
          >
            <FormItem class="flex justify-between items-end space-y-0">
              <FormLabel>{{ t(`fields.${item.label}`) }}</FormLabel>
              <FormControl>
                <Switch :checked="value" @update:checked="handleChange" />
              </FormControl>
            </FormItem>
          </FormField>

          <FormField
            v-slot="{ componentField }"
            :name="`documentValues.client.${item.field}`"
          >
            <FormItem>
              <FormControl>
                <Input
                  :disabled="!values.fields[item.field]"
                  v-bind="componentField"
                />
              </FormControl>
            </FormItem>
          </FormField>
        </div>
      </CardContent>

      <CardFooter>
        <Button type="submit" class="col-span-3">
          {{ t(`buttons.generate-${documentType}`) }}
        </Button>
      </CardFooter>
    </Card>
  </form>
</template>
