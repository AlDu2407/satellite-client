import { Textarea } from "@/components/ui/textarea";
import { SatelliteResponse } from "@/types/generated/response";
import { useEffect, useState } from "react";

interface ResponseAreaProps {
  response: SatelliteResponse | undefined;
}

const ResponseArea = ({ response }: ResponseAreaProps) => {
  const [style, setStyle] = useState<string>("");

  useEffect(() => {
    if (!response?.code) {
      return;
    }
    const code = response.code;
    if (code >= 100 && code < 200) {
      setStyle("text-xl font-extrabold text-yellow-500");
    } else if (code >= 200 && code < 300) {
      setStyle("text-xl font-extrabold text-green-500");
    } else if (code >= 400 && code < 500) {
      setStyle("text-xl font-extrabold text-red-500");
    }
  }, [response]);

  return (
    <div className="h-full">
      <p>
        {response && (
          <span className={style}>
            {response.code} - {response.status}
          </span>
        )}
      </p>
      <Textarea
        className="mt-2 h-[calc(100%-3.5rem)] resize-none overflow-auto text-lg"
        value={response?.body ? JSON.stringify(response.body, null, 2) : ""}
      />
    </div>
  );
};

export default ResponseArea;
