export interface FileInfo {
  name: string;
  path: string;
  size: number;
  extension: string;
  selectedFormat?: string;
}

export interface ConversionTool {
  name: string;
  command: string;
  supportedInputs: string[];
  supportedOutputs: string[];
}

export interface ConversionOption {
  format: string;
  tool: string;
  display_name: string;
  color: string;
}

export interface BatchConversionSettings {
  [inputExtension: string]: {
    format: string;
    isMixed: boolean; // true when files of same type have different target formats
  };
}
