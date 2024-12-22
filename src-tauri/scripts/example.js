butler_log.log("Hello from example.ts");

let content;
try {
  content = await butler_fs.readFile("./log.txt");
  butler_log.log(content);
} catch (e) {
  butler_log.log(e);
}
