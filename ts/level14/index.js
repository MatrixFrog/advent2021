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

// Mimicking .windows2()
//   for (const pair of 'abcd') {
//     console.log(pair); // ['a', 'b'] then ['b', 'c'] then ['c', 'd']
//   }
function* iterateByTwos(iterable) {
  let two = [];
  for (const item of iterable) {
    two.push(item);
    if (two.length > 2) {
      two.shift();
    }
    if (two.length == 2) {
      yield two;
    }
  }
}

function* getInsertions(prevState, rules) {
  for (const [a, b] of iterateByTwos(prevState)) {
    yield rules[a + b];
  }
}

function* interleave(iterable1, iterable2) {
  let i1 = iterable1[Symbol.iterator]();
  let i2 = iterable2[Symbol.iterator]();
  while (true) {
    let { value, done } = i1.next();
    if (done) {
      return { done };
    }
    yield value;
    yield i2.next().value;
  }
}

function apply(prevState, rules) {
  let insertions = getInsertions(prevState, rules);
  return interleave(prevState, insertions);
}

function printState(state) {
  let s = "";
  for (const c of state) {
    s += c;
  }
  console.log(s);
}

function solve(initialState, rules) {
  let state = initialState;
  for (let i = 0; i < 10; i++) {
    printState(state);
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
