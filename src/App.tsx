import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import DropZone from "./components/DropZone";
import ConversionOptions from "./components/ConversionOptions";
import AdvancedSettings from "./components/AdvancedSettings";
import { FileInfo } from "./types";

function App() {
  const [selectedFile, setSelectedFile] = useState<FileInfo | null>(null);
  const [selectedFormat, setSelectedFormat] = useState<string>("");
  const [advancedOptions, setAdvancedOptions] = useState<string>("");
  const [outputDirectory, setOutputDirectory] = useState<string>("");
  const [isConverting, setIsConverting] = useState(false);
  const [conversionProgress, setConversionProgress] = useState(0);
  const [conversionResult, setConversionResult] = useState<{
    success: boolean;
    message: string;
    outputPath?: string;
  } | null>(null);

  const handleFileSelect = (file: FileInfo) => {
    setSelectedFile(file);
    setSelectedFormat("");
    setConversionResult(null);
  };

  const handleConvert = async () => {
    if (!selectedFile || !selectedFormat) return;

    setIsConverting(true);
    setConversionProgress(0);
    setConversionResult(null);

    try {
      const result = await invoke("convert_file", {
        inputPath: selectedFile.path,
        outputFormat: selectedFormat,
        outputDirectory: outputDirectory || undefined,
        advancedOptions: advancedOptions || undefined,
      });

      setConversionResult({
        success: true,
        message: "Conversion completed successfully!",
        outputPath: result as string,
      });
    } catch (error) {
      setConversionResult({
        success: false,
        message: `Conversion failed: ${error}`,
      });
    } finally {
      setIsConverting(false);
      setConversionProgress(100);
    }
  };

  return (
    <div className="min-h-screen bg-background p-8">
      <div className="max-w-4xl mx-auto">
        <header className="text-center mb-12">
          <h1 className="text-4xl font-bold text-primary mb-2">ConvertSave</h1>
          <p className="text-secondary font-normal">Convert any file locally with ease</p>
        </header>

        <main className="space-y-8">
          <DropZone onFileSelect={handleFileSelect} selectedFile={selectedFile} />

          {selectedFile && (
            <>
              <ConversionOptions
                inputFile={selectedFile}
                selectedFormat={selectedFormat}
                onFormatSelect={setSelectedFormat}
              />

              <AdvancedSettings
                advancedOptions={advancedOptions}
                onOptionsChange={setAdvancedOptions}
                outputDirectory={outputDirectory}
                onDirectoryChange={setOutputDirectory}
              />

              <div className="flex justify-center">
                <button
                  onClick={handleConvert}
                  disabled={!selectedFormat || isConverting}
                  className={`
                    btn-chunky px-8 py-4 text-lg
                    ${selectedFormat && !isConverting
                      ? "bg-aquamarine text-dark-purple"
                      : "bg-light-grey text-secondary"
                    }
                  `}
                >
                  {isConverting ? "Converting..." : "Convert"}
                </button>
              </div>

              {isConverting && (
                <div className="w-full bg-light-grey rounded-full h-2">
                  <div
                    className="bg-aquamarine h-2 rounded-full transition-all duration-300"
                    style={{ width: `${conversionProgress}%` }}
                  />
                </div>
              )}

              {conversionResult && (
                <div
                  className={`
                    p-4 rounded-xl font-normal
                    ${conversionResult.success
                      ? "bg-success-bg text-success-text"
                      : "bg-error-bg text-error-text"
                    }
                  `}
                >
                  <p className="font-bold">{conversionResult.message}</p>
                  {conversionResult.outputPath && (
                    <button
                      onClick={() => invoke("open_folder", { path: conversionResult.outputPath })}
                      className="mt-2 underline hover:no-underline"
                    >
                      Open output folder
                    </button>
                  )}
                </div>
              )}
            </>
          )}
        </main>
      </div>
    </div>
  );
}

export default App;
