/* import { readFileSync } from 'node:fs';

function day7() {
  const content = readFileSync('input.txt', 'utf8');
  const equations = content.split('\n').map(parseLine);
  const targets: number[] = [];
  for (const [target, members] of equations) {
    const canBeResolved = resolveTarget(target, members);
    if (canBeResolved) {
      targets.push(target);
    }
  }
  
  // 303766878298 too low
  // 1494403784048172 too high
  // 303766880536 !!!
  return targets.reduce((acc, cur) => acc + cur, 0);
}

function parseLine(line: string): [number, number[]] {
  const map = [];
  const [id, nums] = line.split(':');
  const numbers = nums.trim().split(' ').map(Number);
  map[0] = Number(id);
  map[1] = numbers;
  return map as [number, number[]];
}

function resolveTarget(target: number, members: number[]): boolean {
  const combs = generateCombinations(["+", "*"], members.length - 1);
  for (const comb of combs) {
    const result = applyOperations(members, comb);
    if (result === target) {
      return true;
    }
  }
  
  return false;
}


function generateCombinations(chars: string[], length: number): string[][] {
  const combinations: string[][] = [];

  function generate(index: number, currentCombination: string[]) {
    if (index === length) {
      combinations.push([...currentCombination]);
      return;
    }

    for (const char of chars) {
      currentCombination[index] = char;
      generate(index + 1, currentCombination);
    }
  }

  generate(0, new Array(length));
  return combinations;
}

function applyOperations(members: number[], operations: string[]): number {
  let result = members[0];
  for (let i = 0; i < operations.length; i++) {
    const op = operations[i];
    if (op === '+') {
      result += members[i + 1];
    } else if (op === '*') {
      result *= members[i + 1];
    }
  }
  return result;
}

console.log(day7());
 */




import { readFileSync } from 'node:fs';

function day7() {
  const content = readFileSync('input.txt', 'utf8');
  const equations = content.split('\n').map(parseLine);
  const targets: number[] = [];
  for (const [target, members] of equations) {
    const canBeResolved = resolveTarget(target, members);
    if (canBeResolved) {
      targets.push(target);
    }
  }
  // 337041851384440 !!! First try
  return targets.reduce((acc, cur) => acc + cur, 0);
}

function parseLine(line: string): [number, number[]] {
  const map = [];
  const [id, nums] = line.split(':');
  const numbers = nums.trim().split(' ').map(Number);
  map[0] = Number(id);
  map[1] = numbers;
  return map as [number, number[]];
}

function resolveTarget(target: number, members: number[]): boolean {
  const combs = generateCombinations(["+", "*", "|"], members.length - 1);
  for (const comb of combs) {
    const result = applyOperations(members, comb);
    if (result === target) {
      return true;
    }
  }
  
  return false;
}


function generateCombinations(chars: string[], length: number): string[][] {
  const combinations: string[][] = [];

  function generate(index: number, currentCombination: string[]) {
    if (index === length) {
      combinations.push([...currentCombination]);
      return;
    }

    for (const char of chars) {
      currentCombination[index] = char;
      generate(index + 1, currentCombination);
    }
  }

  generate(0, new Array(length));
  return combinations;
}

function applyOperations(members: number[], operations: string[]): number {
  let result = members[0];
  for (let i = 0; i < operations.length; i++) {
    const op = operations[i];
    if (op === '+') {
      result += members[i + 1];
    } else if (op === '*') {
      result *= members[i + 1];
    } else if (op === '|') {
      result = Number(String(result) + String(members[i + 1]));
    }
  }
  return result;
}

console.log(day7());
