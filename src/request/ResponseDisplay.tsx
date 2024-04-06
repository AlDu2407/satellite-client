interface ResponseDisplayProperties {
  response: string;
}

const ResponseDisplay = (props: ResponseDisplayProperties) => {
  return (
    <div className="flex items-center bg-white px-2 pt-2 text-gray-500 dark:bg-gray-900 dark:text-gray-400">
      <textarea
        className="w-full rounded-lg border border-gray-300 bg-gray-50 p-4 text-base text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
        cols={100}
        rows={10}
        value={props.response}
      />
    </div>
  );
};

export default ResponseDisplay;
