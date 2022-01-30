import { readFileSync } from "fs";

function parseInput() {
  const input = readFileSync("./level14/input.txt", "utf-8");
  let [initialState, _, ...ruleLines] = input.split("\n");
  const rules = parseRules(ruleLines);
  return [initialState, rules];
}

function parseRule(ruleLine) {
  return ruleLine.split(" -> ");
}

function parseRules(ruleLines) {
  let result = {};
  for (const line of ruleLines) {
    const [l, r] = parseRule(line);
    result[l] = r;
  }
  return result;
}

function* applyGen(prevState, rules) {
  for (let i = 0; i < prevState.length - 1; i++) {
    yield prevState[i];
    yield rules[prevState[i] + prevState[i + 1]];
  }
  yield prevState[prevState.length - 1];
}

function apply(prevState, rules) {
  let state = "";
  for (const ch of applyGen(prevState, rules)) {
    state += ch;
  }
  return state;
}

function solve(initialState, rules) {
  let state = initialState;
  for (let i = 0; i < 10; i++) {
    state = apply(state, rules);
  }

  const freq = {};
  for (let ch of state) {
    freq[ch] = (freq[ch] || 0) + 1;
  }
  const frequencies = Object.values(freq);
  frequencies.sort((a, b) => a - b);
  console.log(frequencies[frequencies.length - 1] - frequencies[0]);
}

function main() {
  const [initialState, rules] = parseInput();
  solve(initialState, rules);
}

main();
