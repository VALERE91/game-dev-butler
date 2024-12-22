const { core } = Deno;

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

globalThis.butler_log = {
  info: (...args) => {
    core.print(`[info]: ${argsToMessage(...args)}\n`, false);
  },
  error: (...args) => {
    core.print(`[err]: ${argsToMessage(...args)}\n`, true);
  },
  log: (msg) => {
    core.ops.op_log(msg);
  },
};
