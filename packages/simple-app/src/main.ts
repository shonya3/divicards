import init, { create_sample_from_csv } from "@divicards/divi-wasm";

async function main() {
  await init();

  const csv = `name,amount
The Doctr,2
Rain of Chaos,30`;

  const sample = create_sample_from_csv(csv);
  console.log("Sample:", sample);
}

main();
