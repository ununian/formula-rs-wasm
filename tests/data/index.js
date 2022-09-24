const pre = require('./pre.json');
const sit2 = require('./sit2.json');

console.log(Array.from(new Set(([...pre, ...sit2]).map(p => p.expression))).join('\r\n'));
