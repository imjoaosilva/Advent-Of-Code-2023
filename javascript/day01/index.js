// import fs from "fs";
//
// function firstPart() {
//   const file = process.argv.includes("--test")
//     ? "./initial1.txt"
//     : "./input1.txt";
//   const codes = fs.readFileSync(file, "utf8").trim().split("\n");
//   const regex = /\d+/g;
//   return codes.reduce((acc, curr) => {
//     const numbers = curr.match(regex).join("").split("");
//     return parseInt(numbers[0] + numbers[numbers.length - 1]) + acc;
//   }, 0);
// }
//
// console.log(firstPart());
//
// function secondPart() {
//   const file = process.argv.includes("--test")
//     ? "./initial1.txt"
//     : "./input1.txt";
//   const codes = fs.readFileSync(file, "utf8").trim().split("\n");
// }
//
// console.log(secondPart());
console.log("Hello world, sou um merda1");
