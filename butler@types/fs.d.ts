declare module "butler::fs" {
  export const Filesystem: {
    readFile: (path: String) => String;
    writeFile: (path: String, content: String) => void;
    removeFile: (path: string) => void;
  };
}
