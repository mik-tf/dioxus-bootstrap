#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

import { transformSource } from './migrate-bootstrap-rsx.mjs';

const ROOT = path.dirname(fileURLToPath(import.meta.url));
const FIXTURES = path.join(ROOT, 'fixtures');

function read(name) {
  return fs.readFileSync(path.join(FIXTURES, name), 'utf8').replace(/\r\n/g, '\n');
}

function assertEqual(name, actual, expected) {
  if (actual !== expected) {
    console.error(`FAIL ${name}`);
    console.error('--- actual ---');
    console.error(actual);
    console.error('--- expected ---');
    console.error(expected);
    process.exit(1);
  }
}

const common = transformSource(read('common.input.rs'), 'common.input.rs');
assertEqual('common conversion changed flag', String(common.changed), 'true');
assertEqual('common warnings', JSON.stringify(common.warnings), '[]');
assertEqual('common conversion', common.source, read('common.expected.rs'));

const imports = transformSource(read('imports.input.rs'), 'imports.input.rs');
assertEqual('imports conversion changed flag', String(imports.changed), 'true');
assertEqual('imports warnings', JSON.stringify(imports.warnings), '[]');
assertEqual('imports conversion', imports.source, read('imports.expected.rs'));

const dynamic = transformSource(read('dynamic.input.rs'), 'dynamic.input.rs');
assertEqual('dynamic unchanged', dynamic.source, read('dynamic.input.rs'));
assertEqual('dynamic warning count', String(dynamic.warnings.length), '1');
assertEqual('dynamic warning kind', dynamic.warnings[0].kind, 'manual_review');

const dynamicUtility = transformSource(read('dynamic-utility.input.rs'), 'dynamic-utility.input.rs');
assertEqual('dynamic utility unchanged', dynamicUtility.source, read('dynamic-utility.input.rs'));
assertEqual('dynamic utility warnings', JSON.stringify(dynamicUtility.warnings), '[]');

console.log('OK migrate-bootstrap-rsx fixtures passed');
