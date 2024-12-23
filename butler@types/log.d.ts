declare module "butler::log" {
  export const Logger: {
    info: (...args) => void;
    error: (...args) => void;
    log: (message: string) => void;
  };
}
