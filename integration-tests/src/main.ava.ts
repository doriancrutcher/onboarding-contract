import { Worker, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const evaluator = await root.createSubAccount("contract-checker");
  const test_taker = await root.createSubAccount("test-taker");
  const hello_near = await root.createSubAccount("hello-near");
  // Get wasm file path from package.json test script in folder above
  await evaluator.deploy(process.argv[2]);
  await test_taker.deploy(process.argv[3]);
  await hello_near.deploy(process.argv[4]);

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, evaluator, test_taker, hello_near };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("Check Hello Near Test", async (t) => {
  const { evaluator, hello_near } = t.context.accounts;
  let output = await evaluator.call(
    evaluator,
    "evaluate_hello_near",
    {
      contract_name: hello_near.accountId,
    },
    { gas: "300000000000000" }
  );
  t.true(output);
});

test("Contract Checker will test test taking contract's lookup Map", async (t) => {
  const { evaluator, test_taker } = t.context.accounts;
  let output = await evaluator.call(
    evaluator,
    "evaluate_map",
    {
      contract_name: test_taker.accountId,
    },
    { gas: "300000000000000" }
  );
  t.true(output);
});

test("Contract Checker will test the contract's vector implementation", async (t) => {
  const { evaluator, test_taker } = t.context.accounts;
  let output = await evaluator.call(
    evaluator,
    "evaluate_check_collection_test_vector",
    {
      contract_name: test_taker.accountId,
    },
    { gas: "300000000000000" }
  );
  console.log("vector output is ", output);
  t.true(output);
});

test("contract can store a value in the lookup map", async (t) => {
  const { root, evaluator, test_taker } = t.context.accounts;
  await test_taker.call(test_taker, "add_to_map", {
    key: "test",
    value: "fen",
  });
  let result = await test_taker.view("get_from_map", {
    key: "test",
  });
  t.is(result, "fen");
});
