// Checks supported opcodes against the authors' downloaded ISA interpreter.
// Neither implementation is a full Magic rules engine.
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { runInNewContext } from 'node:vm';
import assert from 'node:assert/strict';

const LanguageEngine = runInNewContext(
  readFileSync('references/MTGProgSimulatorText.js', 'utf8') + '\nLanguageEngine;',
  { console },
);
function rust(...args) {
  const result = spawnSync('target/debug/mtg-computer', args, { encoding: 'utf8' });
  assert.equal(result.status, 0, result.stderr);
  return result.stdout;
}
for (const file of ['examples/cube.mtg', 'examples/sum.mtg', 'examples/big-number.mtg']) {
  const opcodes = [...rust('compile', file).matchAll(/\| (\d+) (\d+) (\d+):/g)]
    .map(match => match.slice(1).join(' '));
  const reference = new LanguageEngine(opcodes.join('\n'));
  const frames = rust('trace', file).trim().split('\n').map(line => JSON.parse(line));
  for (const frame of frames) {
    assert.equal(reference.programCounter, frame.pc, `${file}: instruction address`);
    reference.executeCurrentInstruction();
    assert.deepEqual(Array.from(reference.registers, String), frame.registers_after, `${file}: registers at step ${frame.step}`);
    assert.equal(reference.flag, frame.flag_after, `${file}: flag at step ${frame.step}`);
    assert.equal(reference.isEnded(), frame.halted, `${file}: halt state`);
    if (!frame.halted) assert.equal(reference.programCounter, frame.next_pc, `${file}: branch target`);
  }
  console.log(`${file}: ${frames.length} instruction snapshots agree`);
}
