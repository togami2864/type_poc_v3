async function returnsPromise(): Promise<string> {
  return "value";
}
returnsPromise().then(() => {});
