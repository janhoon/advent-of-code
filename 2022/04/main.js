const fs = require("fs");
const readline = require("readline");

const main = async () => {
  if (process.argv.length < 3) {
    console.log("Usage: node main.js <inputfile>");
    return;
  }

  const input = fs.createReadStream(process.argv[2]);

  const rl = readline.createInterface({
    input,
    crlfDelay: Infinity,
  });

  let overlappingPairs = 0;
  for await (const line of rl) {
    const [group1, group2] = line.split(",");
    const [g1Start, g1End] = group1.split("-").map(Number);
    const [g2Start, g2End] = group2.split("-").map(Number);

    // find complete overlaps (challenge 1)
    // if (g1Start <= g2Start && g1End >= g2End) {
    //   isPair = true;
    //   overlappingPairs++;
    // } else if (g2Start <= g1Start && g2End >= g1End) {
    //   isPair = true;
    //   overlappingPairs++;
    // }

    // find all overlaps (challenge 2)
    if (g1Start <= g2Start && g1End >= g2Start) {
      overlappingPairs++;
    } else if (g2Start <= g1Start && g2End >= g1Start) {
      overlappingPairs++;
    }
  }

  console.log("Overlapping pairs:", overlappingPairs);
};

main();
