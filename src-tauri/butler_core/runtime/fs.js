const { core } = Deno;

export class Filesystem {
  static readFile(path) {
    return core.ops.op_read_file(path);
  }

  static writeFile(path, contents) {
    core.ops.op_write_file(path, contents);
  }

  static removeFile(path) {
    core.ops.op_remove_file(path);
  }
}
