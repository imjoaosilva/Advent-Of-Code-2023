import fs from "fs";

function firstPart() {
  const file = process.argv.includes("--test")
    ? "./initial1.txt"
    : "./input1.txt";
  const codes = fs.readFileSync(file, "utf8").trim().split("\n");
  const regex = /\d+/g;

  return codes.reduce((acc, curr) => {
    const numbers = curr.match(regex).join("").split("");
    return parseInt(numbers[0] + numbers[numbers.length - 1]) + acc;
  }, 0);
}

console.log(firstPart());

function secondPart() {
  const file = process.argv.includes("--test")
    ? "./initial2.txt"
    : "./input2.txt";
  const codes = fs.readFileSync(file, "utf8").trim().split("\n");

  const stringNumbers = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
  ];

  return codes.reduce((acc, curr) => {
    const characters = curr
      .split("")
      .map((character, index) => {
        const matchedStringNumber = stringNumbers.find((stringNum) => {
          const string = curr.slice(index, index + stringNum.length);
          return string === stringNum;
        });

        if (matchedStringNumber)
          return stringNumbers.indexOf(matchedStringNumber) + 2;

        return Number.isInteger(parseInt(character)) ? character : undefined;
      })
      .filter((char) => char !== undefined);

    const numbers = characters.join("").split("");

    return parseInt(numbers[0] + numbers[numbers.length - 1]) + acc;
  }, 0);
}

console.log(secondPart());
