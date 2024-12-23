Butler.log("Hello from example.ts");

let content;
try {
  content = await Butler.readFile("./log.txt");
  Butler.log(content);
} catch (e) {
  Butler.log(e);
}
