export interface FileInfo {
  name: string;
  path: string;
  size: number;
  extension: string;
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
