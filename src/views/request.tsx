import { Commands, isSatelliteErr } from "@/commands/tauriCommands";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { Request } from "@/types/generated/request";
import { zodResolver } from "@hookform/resolvers/zod";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";

const urlForm = z.object({
  url: z.string().url(),
  body: z.string(),
});

type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";

const SUPPORTED_HTTP_METHODS: HttpMethod[] = [
  "GET",
  "POST",
  "PUT",
  "DELETE",
] as const;

const RequestView = () => {
  const [result, setResult] = useState<any | undefined>(undefined);
  const [method, setMethod] = useState<HttpMethod>(SUPPORTED_HTTP_METHODS[0]);
  const [responseText, setResponseText] = useState<string>("");

  const form = useForm<z.infer<typeof urlForm>>({
    resolver: zodResolver(urlForm),
    defaultValues: {
      url: "https://jsonplaceholder.typicode.com/todos/1",
      body: "",
    },
  });

  async function onSubmit({ url, body }: z.infer<typeof urlForm>) {
    try {
      const request = (
        canContainBody()
          ? {
              method,
              url,
              body: JSON.parse(body),
              secure: true,
            }
          : { method, url, secure: true }
      ) as Request;

      const response = await Commands.request(request);
      setResult(response);
    } catch (err) {
      if (isSatelliteErr(err)) {
        setResult(err);
      } else {
        setResult(undefined);
      }
    }
  }

  const canContainBody = () => {
    return method === "POST" || method === "PUT";
  };

  useEffect(() => {
    if (result) {
      setResponseText(JSON.stringify(result, undefined, 2));
    } else {
      setResponseText("");
    }
  }, [result]);

  return (
    <div className="flex h-full flex-col">
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          className="mb-8 flex w-full flex-col items-center"
        >
          <div className="mb-4 flex w-full flex-row items-center">
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline">{method}</Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="start">
                {SUPPORTED_HTTP_METHODS.map((httpMethod) => (
                  <DropdownMenuItem
                    key={httpMethod}
                    onClick={() => setMethod(httpMethod)}
                  >
                    {httpMethod}
                  </DropdownMenuItem>
                ))}
              </DropdownMenuContent>
            </DropdownMenu>
            <FormField
              control={form.control}
              name="url"
              render={({ field }) => (
                <>
                  <FormItem className="mx-2 flex w-full items-center">
                    <FormLabel className="text-lg">Url</FormLabel>
                    <FormControl>
                      <Input placeholder="request url..." {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                </>
              )}
            />
            <Button type="submit">Execute</Button>
          </div>
          <div className="w-full items-center">
            <Tabs defaultValue="parameters">
              <TabsList>
                <TabsTrigger value="parameters" className="relative">
                  Parameters
                </TabsTrigger>
                <TabsTrigger value="headers">Headers</TabsTrigger>
                <TabsTrigger value="body">Body</TabsTrigger>
              </TabsList>
              <TabsContent
                value="parameters"
                className="border-none p-0 outline-none"
              >
                <div>Parameters Tab</div>
              </TabsContent>
              <TabsContent
                value="headers"
                className="h-full flex-col border-none p-0 data-[state=active]:flex"
              >
                <div>Headers Tab</div>
              </TabsContent>
              <TabsContent value="body" className="w-full flex-col border-none">
                <FormField
                  control={form.control}
                  name="body"
                  render={({ field }) => (
                    <FormItem className="grow rounded-lg bg-muted p-4">
                      <FormLabel htmlFor="request-payload">Body</FormLabel>
                      <FormControl>
                        <Textarea id="request-payload" {...field} />
                      </FormControl>
                    </FormItem>
                  )}
                />
              </TabsContent>
            </Tabs>
          </div>
        </form>
      </Form>
      <div className="grow overflow-y-scroll rounded-lg bg-muted p-4">
        <Label htmlFor="response-payload">Response</Label>
        <Textarea
          id="reponse-content"
          className="h-full"
          value={responseText}
        />
      </div>
    </div>
  );
};

export default RequestView;
