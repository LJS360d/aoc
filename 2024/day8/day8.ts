import { readFileSync } from 'node:fs';

function day8() {
  const content = readFileSync('input.txt', 'utf8');
  const grid = parseGrid(content);
  // 50x50
  const frequencies = getAllFrequencies(grid);
  const antinodes = getAllAntinodesPositions(grid, frequencies);
  return visualizeAntinodes(grid, antinodes)
}

function parseGrid(content: string): string[][] {
  return content.split('\n').map((s) => s.split(''));
}

function getAllFrequencies(grid: string[][]): Map<string, number[][]> {
  const frequencies = new Map<string, number[][]>();
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
      const current = grid[i][j];
      if (!current || current === '.') {
        continue;
      }
      if (!frequencies.has(current)) {
        frequencies.set(current, []);
      }
      const currentFrequenciesPositions = frequencies.get(current)!;
      currentFrequenciesPositions.push([i, j]);
    }
  }
  return frequencies;
}

function getAllAntinodesPositions(
  grid: string[][],
  frequencies: Map<string, number[][]>
): number[][] {
  const allAntinodesPositions: number[][] = [];
  for (const frequencyWithPositions of frequencies.entries()) {
    const antinodes = getFrequencyAntinodes(grid, frequencyWithPositions);
    allAntinodesPositions.push(...antinodes);
  }
  return allAntinodesPositions;
}

function getFrequencyAntinodes(
  grid: string[][],
  frequencyWithPositions: [string, number[][]]
): number[][] {
  const [, positions] = frequencyWithPositions;
  const antinodes: number[][] = [];

  for (const pos of positions) {
    for (const nextPos of positions.filter((p) => p !== pos)) {
      const dist = getGridDistance(pos, nextPos);
      antinodes.push(nextPos)
      const antinodes1 = getResonantAntinodes(grid, pos, dist);
      antinodes.push(...antinodes1.filter((a) => !antinodes.some((b) => b[0] === a[0] && b[1] === a[1])));
      const antinodes2 = getResonantAntinodes(grid, nextPos, [-dist[0], -dist[1]]);
      antinodes.push(...antinodes2.filter((a) => !antinodes.some((b) => b[0] === a[0] && b[1] === a[1])));
    }
    antinodes.push(pos);
  }
  return antinodes
}

function isValidGridPosition(grid: string[][], [x, y]: number[]): boolean {
  return x >= 0 && x < grid.length && y >= 0 && y < grid[0].length;
}

function getGridDistance(a: number[], b: number[]): number[] {
  const [ax, ay] = a;
  const [bx, by] = b;
  return [ax - bx, ay - by];
}

function getResonantAntinodes(grid: string[][], pos: number[], dist: number[]): number[][] {
  // keep creating antinodes until we reach the border of the grid
  const antinodes: number[][] = [];
  let [x, y] = pos;
  while (isValidGridPosition(grid, [x, y])) {
    antinodes.push([x, y]);
    [x, y] = [x + dist[0], y + dist[1]];
  }
  return antinodes;

}

function visualizeAntinodes(_grid: string[][], antinodes: number[][]): number {
  const grid = [..._grid]
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
      const antinodePos = antinodes.find((pos) => pos[0] === i && pos[1] === j);
      if (antinodePos) {
        grid[i][j] = '#';
      }
    }
    
  }
  const withAntinodes = grid.map((row) => row.join('')).join('\n');
  console.log(withAntinodes);  
  return withAntinodes.match(/#/g)?.length || 0
  
}

console.log(day8());




/* 
import { readFileSync } from 'node:fs';

function day8() {
  const content = readFileSync('input.txt', 'utf8');
  const grid = parseGrid(content);
  // 50x50
  const frequencies = getAllFrequencies(grid);
  const antinodes = getAllAntinodesPositions(grid, frequencies);
  return visualizeAntinodes(grid, antinodes)
}

function parseGrid(content: string): string[][] {
  return content.split('\n').map((s) => s.split(''));
}

function getAllFrequencies(grid: string[][]): Map<string, number[][]> {
  const frequencies = new Map<string, number[][]>();
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
      const current = grid[i][j];
      if (!current || current === '.') {
        continue;
      }
      if (!frequencies.has(current)) {
        frequencies.set(current, []);
      }
      const currentFrequenciesPositions = frequencies.get(current)!;
      currentFrequenciesPositions.push([i, j]);
    }
  }
  return frequencies;
}

function getAllAntinodesPositions(
  grid: string[][],
  frequencies: Map<string, number[][]>
): number[][] {
  const allAntinodesPositions: number[][] = [];
  for (const frequencyWithPositions of frequencies.entries()) {
    const antinodes = getFrequencyAntinodes(grid, frequencyWithPositions);
    allAntinodesPositions.push(...antinodes);
  }
  return allAntinodesPositions;
}

function getFrequencyAntinodes(
  grid: string[][],
  frequencyWithPositions: [string, number[][]]
): number[][] {
  const [, positions] = frequencyWithPositions;
  const antinodes: number[][] = [];

  for (const pos of positions) {
    for (const nextPos of positions.filter((p) => p !== pos)) {
      const dist = getGridDistance(pos, nextPos);
      const [x, y] = pos;
      const antinode1 = [x + dist[0], y + dist[1]];
      if (isValidGridPosition(grid, antinode1) && !antinodes.some((a) => a[0] === antinode1[0] && a[1] === antinode1[1])) {
        antinodes.push(antinode1);
      }
      const [nx, ny] = nextPos;
      const antinode2 = [nx - dist[0], ny - dist[1]];
      if (isValidGridPosition(grid, antinode2)  && !antinodes.some((a) => a[0] === antinode2[0] && a[1] === antinode2[1])) {
        antinodes.push(antinode2);
      }
    }
  }  
  return antinodes
}

function isValidGridPosition(grid: string[][], [x, y]: number[]): boolean {
  return x >= 0 && x < grid.length && y >= 0 && y < grid[0].length;
}

function getGridDistance(a: number[], b: number[]): number[] {
  const [ax, ay] = a;
  const [bx, by] = b;
  return [ax - bx, ay - by];
}

function visualizeAntinodes(_grid: string[][], antinodes: number[][]): number {
  const grid = [..._grid]
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
      const antinodePos = antinodes.find((pos) => pos[0] === i && pos[1] === j);
      if (antinodePos) {
        grid[i][j] = '#';
      }
    }
    
  }
  const withAntinodes = grid.map((row) => row.join('')).join('\n');
  console.log(withAntinodes);  
  return withAntinodes.match(/#/g)?.length || 0
  
}

console.log(day8());
 */