async function test(): Promise<void> {
  Promise.resolve("value");
  Promise.resolve("value").then(() => {});
  Promise.resolve("value").catch();
  Promise.resolve("value").finally();
}

async function returnPromise(): Promise<string> {
  return "value";
}
// invalid
returnPromise();
returnPromise().then(() => {});
returnPromise().catch();
returnPromise().finally();

//valid
await returnPromise2();
(async () => {
  await returnPromise2();
  await returnPromise2().then(() => {});
  await returnPromise2().catch();
  await returnPromise2().finally();
})();

// invalid
declare const promiseValue: Promise<number>;
async function test4(): Promise<void> {
  promiseValue;
  promiseValue.then(() => {});
  promiseValue.catch();
  //valid
  await promiseValue.finally();
}
test4();

// invalid
declare const promiseOrNumber: Promise<number> | number;

async function test() {
  promiseOrNumber;
}

// invalid
const foo = async (): Promise<void> => {
  Promise.resolve("value");
};
foo();
