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

declare const maybeCallable: string | (() => void);
Promise.resolve().then(() => {}, maybeCallable);

declare const maybeCallable2: string | (() => void);
declare const definitelyCallable: () => void;
Promise.resolve().then(() => {}, undefined);
Promise.resolve().then(() => {}, null);
Promise.resolve().then(() => {}, 3);
Promise.resolve().then(() => {}, maybeCallable2);
Promise.resolve().then(() => {}, definitelyCallable);

Promise.resolve().catch(undefined);
Promise.resolve().catch(null);
Promise.resolve().catch(3);
await Promise.resolve().catch(maybeCallable2);
Promise.resolve().catch(definitelyCallable);
