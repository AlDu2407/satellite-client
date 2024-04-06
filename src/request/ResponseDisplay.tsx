interface ResponseDisplayProperties {
    response: any;
}

const ResponseDisplay = (props: ResponseDisplayProperties) => {
    return (
        <div className="flex h-full w-full items-center bg-white px-2 py-2 text-gray-500 dark:bg-gray-900 dark:text-gray-400">
            <textarea
                className="h-full w-full rounded-lg border border-gray-300 bg-gray-50 p-4 text-base text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
                value={JSON.stringify(props.response, undefined, 2)}
            />
        </div>
    );
};

export default ResponseDisplay;
