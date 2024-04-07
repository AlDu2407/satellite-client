import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { zodResolver } from "@hookform/resolvers/zod";
import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";

const urlForm = z.object({
  url: z.string().url(),
});

const RequestView = () => {
  const [result, setResult] = useState<any | undefined>(undefined);

  const form = useForm<z.infer<typeof urlForm>>({
    resolver: zodResolver(urlForm),
    defaultValues: {
      url: "",
    },
  });

  async function onSubmit({ url }: z.infer<typeof urlForm>) {
    const result = await invoke("execute_request", { url: url });
    setResult(result);
  }

  return (
    <div className="flex h-full flex-col">
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          className="mb-8 flex w-full items-center space-x-4"
        >
          <FormField
            control={form.control}
            name="url"
            render={({ field }) => (
              <FormItem className=" flex w-full items-center space-x-4 space-y-0">
                <FormLabel className="text-lg">Url</FormLabel>
                <FormControl>
                  <Input placeholder="request url..." {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type="submit">Execute</Button>
        </form>
      </Form>
      <div className="grow overflow-y-scroll rounded-lg bg-muted p-4">
        <code>{JSON.stringify(result, undefined, 2)}</code>
      </div>
    </div>
  );
};

export default RequestView;
