import test from 'ava'

import { getProcessCommandLine } from '../index.js'

test('returns the process command line', (t) => {
  t.is(getProcessCommandLine("explorer.exe").toLowerCase(), "c:\\windows\\explorer.exe")
})

test('throws an error when the process is not running', (t) => {
  const error = t.throws(() => getProcessCommandLine("RiotClientUx.exe"), {instanceOf: Error})
  t.is(error.code, 'GenericFailure')
  t.is(error.message, 'Process is not running')
})
