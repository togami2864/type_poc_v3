async function returnsPromise(): Promise<string> {
  return "value";
}
returnsPromise().then(() => {});

async function test(): Promise<void> {
  Promise.resolve("value");
  Promise.resolve("value").then(() => {});
  Promise.resolve("value").catch();
  Promise.resolve("value").finally();
}

async function test2(): Promise<void> {
  Promise.resolve("value");
  Promise.resolve("value").then(() => {});
  Promise.resolve("value").finally();
}

async function test3(): Promise<void> {
  Promise.resolve("value");
  Promise.resolve("value").then(() => {});
  await Promise.resolve("value").catch();
  Promise.resolve("value").finally();
}

declare const promiseValue: Promise<number>;
async function test4(): Promise<void> {
  promiseValue;
  promiseValue.then(() => {});
  promiseValue.catch();
  await promiseValue.finally();
}

test4();
