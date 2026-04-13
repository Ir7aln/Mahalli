<script setup lang="ts">
import { commands, type NewClient } from "@/bindings";
import { toTypedSchema } from "@vee-validate/zod";
import * as Logger from "@tauri-apps/plugin-log";
import { useForm } from "vee-validate";
import { toast } from "vue-sonner";
import { z } from "zod";

const { t } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const CreateClientSchema = z.object({
  id: z.string().optional(),
  full_name: z.string().min(2).max(50),
  email: z.string().optional(),
  phone_number: z.string().optional(),
  address: z.string().optional(),
  image: z.string().optional(),
  credit: z.number().optional(),
});

const clientSchema = toTypedSchema(CreateClientSchema);

const form = useForm({
  validationSchema: clientSchema,
});

const image = ref<string | null>(null);

async function createNewClient(client: NewClient) {
  try {
    const result = await commands.createClient({
      full_name: client.full_name,
      email: client.email ?? null,
      phone_number: client.phone_number ?? null,
      address: client.address ?? null,
      image: image.value ?? null,
    });
    if (result.status === "error") throw result.error;
    //
    Logger.info(
      `CREATE CLIENT: ${JSON.stringify({
        ...client,
        image: image.value,
      })}`,
    );
    //
    toast.success(t("notifications.client.created", { name: client.full_name }), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-create-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR CREATE CLIENT: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}

const onSubmit = form.handleSubmit((values) => {
  createNewClient({
    full_name: values.full_name,
    email: values.email ?? null,
    phone_number: values.phone_number ?? null,
    address: values.address ?? null,
    image: null,
  });
});

function setImage(imagePath: string | null) {
  image.value = imagePath;
}

function cleanImage() {
  image.value = null;
}
</script>

<template>
  <form class="w-full flex justify-center" @submit="onSubmit">
    <Card class="w-4/6 lg:w-1/2">
      <CardHeader>
        <CardTitle> {{ t("titles.clients.create") }} </CardTitle>
      </CardHeader>
      <CardContent>
        <UiUploader
          name="Image"
          :extensions="['png', 'jpeg', 'webp', 'jpg']"
          @clear="cleanImage"
          @save-path="setImage"
        />
        <FormField v-slot="{ componentField }" name="full_name">
          <FormItem>
            <FormLabel>{{ t("fields.full-name") }}</FormLabel>
            <FormControl>
              <Input :placeholder="t('fields.full-name')" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="email">
          <FormItem>
            <FormLabel>{{ t("fields.email") }}</FormLabel>
            <FormControl>
              <Input placeholder="example@gmail.com" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="phone_number">
          <FormItem>
            <FormLabel>{{ t("fields.phone") }}</FormLabel>
            <FormControl>
              <Input placeholder="+2126********" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="address">
          <FormItem>
            <FormLabel>{{ t("fields.address") }}</FormLabel>
            <FormControl>
              <Input :placeholder="t('fields.address')" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
      </CardContent>
      <CardFooter>
        <Button type="button" variant="outline" @click="close">
          {{ t("buttons.cancel") }}
        </Button>
        <Button type="submit" class="col-span-2">
          {{ t("buttons.add") }}
        </Button>
      </CardFooter>
    </Card>
  </form>
</template>
