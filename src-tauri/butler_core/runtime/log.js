const { core } = Deno;

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

export class Logger {
  static info(...args) {
    core.print(`[info]: ${argsToMessage(...args)}\n`, false);
  }

  static error(...args) {
    core.print(`[err]: ${argsToMessage(...args)}\n`, true);
  }

  static log(msg) {
    core.ops.op_log(msg);
  }
}
