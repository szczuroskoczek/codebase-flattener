#!/usr/bin/env node
const { spawnSync } = require('child_process')
const { join } = require('path')

const platform = process.platform
let exeName
if (platform === 'win32') exeName = 'windows.exe'
else if (platform === 'darwin') throw new Error('macOS is not supported yet')
else exeName = 'linux'

const binPath = join(__dirname, exeName)
const result = spawnSync(binPath, process.argv.slice(2), { stdio: 'inherit' })
process.exit(result.status)
