import { ChangeEventHandler, MouseEventHandler } from "react";

interface UrlInputProps {
  url: string;
  onChange: ChangeEventHandler<HTMLInputElement>;
  onClick: MouseEventHandler<HTMLButtonElement>;
}

const UrlInput = (props: UrlInputProps) => {
  return (
    <div className="flex items-center gap-x-2 space-x-2 bg-white px-2 text-gray-500 dark:bg-gray-900 dark:text-gray-400">
      <label
        htmlFor="url-input"
        className="mb-2 text-lg font-medium text-gray-900 dark:text-white"
      >
        URL
      </label>
      <input
        type="text"
        id="url-input"
        className="w-full rounded-lg border border-gray-300 bg-gray-50 p-4 text-base text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
        value={props.url}
        onChange={props.onChange}
      />
      <button
        type="button"
        className="mb-2 me-2 rounded-lg border border-gray-300 bg-white px-5 py-2.5 text-sm font-medium text-gray-900 hover:bg-gray-100 focus:outline-none focus:ring-4 focus:ring-gray-100 dark:border-gray-600 dark:bg-gray-800 dark:text-white dark:hover:border-gray-600 dark:hover:bg-gray-700 dark:focus:ring-gray-700"
        onClick={props.onClick}
      >
        Execute
      </button>
    </div>
  );
};

export default UrlInput;
