import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useEffect, useState } from "react";

interface ParameterAreaProps {
  url: string | undefined;
  updateUrl: (url: string) => void;
}

type ParameterType = "query" | "path";

interface Parameter {
  type: ParameterType;
  name: string;
  value: string;
  startIndex: number;
  endIndex: number;
}

const extractParameters = (url: string | undefined): Parameter[][] => {
  if (!url) return [];

  // Find all {param} occurrences
  const matches = [...url.matchAll(/{(.*?)}/g)];
  if (!matches.length) return [];

  const [pathPart, queryPart] = url.split("?");
  const pathParams: Parameter[] = [];
  const queryParams: Parameter[] = [];

  matches.forEach((match) => {
    const paramName = match[1];
    const matchIndex = match.index ?? 0;
    const isPath = !queryPart || matchIndex < pathPart.length;

    const param: Parameter = {
      type: isPath ? "path" : "query",
      name: paramName,
      value: "",
      startIndex: matchIndex,
      endIndex: matchIndex + match[0].length,
    };

    if (isPath) {
      pathParams.push(param);
    } else {
      queryParams.push(param);
    }
  });

  return [pathParams, queryParams];
};

const ParameterArea = ({ url, updateUrl }: ParameterAreaProps) => {
  const [queryParams, setQueryParams] = useState<Parameter[]>([]);
  const [pathParams, setPathParams] = useState<Parameter[]>([]);

  useEffect(() => {
    const [pathParams, queryParams] = extractParameters(url);
    setPathParams(pathParams);
    setQueryParams(queryParams);
  }, [url]);

  const removePathParam = (parameter: Parameter) => {
    setPathParams(pathParams.filter((p) => p !== parameter));
  };

  const removeQueryParam = (parameter: Parameter) => {
    setQueryParams(queryParams.filter((p) => p !== parameter));
  };

  return (
    <>
      <div className="w-full">
        <Label className="mb-2 block text-lg font-medium text-gray-700">
          Path
        </Label>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[40%]">Name</TableHead>
              <TableHead className="w-[50%]">Value</TableHead>
              <TableHead></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {pathParams &&
              pathParams.map((parameter) => (
                <TableRow
                  key={`${parameter.type}-${parameter.name}-${parameter.value}`}
                >
                  <TableCell>{parameter.name}</TableCell>
                  <TableCell>{parameter.value}</TableCell>
                  <TableCell>
                    <Button
                      variant="destructive"
                      onClick={() => removePathParam(parameter)}
                    >
                      Remove
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
          </TableBody>
        </Table>
      </div>
      <div className="w-full">
        <Label className="mb-2 block text-lg font-medium text-gray-700">
          Query
        </Label>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[40%]">Name</TableHead>
              <TableHead className="w-[50%]">Value</TableHead>
              <TableHead></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {queryParams &&
              queryParams.map((parameter) => (
                <TableRow
                  key={`${parameter.type}-${parameter.name}-${parameter.value}`}
                >
                  <TableCell>{parameter.name}</TableCell>
                  <TableCell>{parameter.value}</TableCell>
                  <TableCell>
                    <Button
                      variant="destructive"
                      onClick={() => removeQueryParam(parameter)}
                    >
                      Remove
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
          </TableBody>
        </Table>
      </div>
    </>
  );
};

export default ParameterArea;
