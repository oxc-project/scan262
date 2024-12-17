const es5 = require('./compat-table/data-es5.js').tests.map((t) => ({
  target: 'es5',
  ...t,
}));
const es6 = require('./compat-table/data-es6.js').tests.map((t) => ({
  target: 'es6',
  ...t,
}));
const es2016Plus = require('./compat-table/data-es2016plus.js').tests.map((t) => ({
  target: 'es2016',
  ...t,
}));
const esnext = require('./compat-table/data-esnext.js').tests.map((t) => ({
  target: 'esnext',
  ...t,
}));

const data = es5.concat(es6, es2016Plus, esnext).map((test) => {
  return mapper(test);
});

console.log(JSON.stringify(data, null, 2));

function mapper(test) {
  return {
    name: test.name,
    target: test.target,
    category: test.category,
    significance: test.significance,
    spec: test.spec,
    mdn: test.mdn,
    exec: test.exec && testScript(test.exec).trim(),
    subtests: test.subtests && test.subtests.map(mapper),
  };
}

function testScript(fn) {
  function deindentFunc(fn) {
    fn = fn + '';
    var indent = /(?:^|\n)([\t ]+)[^\n]+/.exec(fn);
    if (indent) {
      fn = fn.replace(new RegExp('\n' + indent[1], 'g'), '\n');
    }
    return fn;
  }
  if (!fn) {
    return '';
  }
  if (typeof fn === 'function') {
    // see if the code is encoded in a comment
    var expr = (fn + '').match(/[^]*\/\*([^]*)\*\/\}$/);
    // if there wasn't an expression, make the function statement into one
    if (!expr) {
      expr = deindentFunc(fn);
      return expr;
    } else {
      expr = deindentFunc(expr[1]);
      return expr;
    }
  } else {
    // it's an array of objects like the following:
    // { type: 'application/javascript;version=1.8', script: function () { ... } }
    return fn.reduce(function(text, script) {
      var expr = deindentFunc(
        (script.script + '').replace(/^function \(\) \{\s*|\s*\}$/g, ''),
      );
      return text + '\n' + expr;
    }, '');
  }
}
