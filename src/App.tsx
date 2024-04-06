import { ThemeProvider } from "@/components/theme/provider";
import RequestView from "@/views/request";
import { ModeToggle } from "./components/theme/toggle";

const App = () => {
  return (
    <ThemeProvider defaultTheme="light" storageKey="satellite-client-ui-theme">
      <header className="flex h-[10%] w-full justify-end px-4 py-2">
        <ModeToggle />
      </header>
      <main className="h-[90%] p-4">
        <RequestView />
      </main>
    </ThemeProvider>
  );
};

export default App;
