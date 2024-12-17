const es5 = require('./compat-table/data-es5.js');
const es6 = require('./compat-table/data-es6.js');
const es2016Plus = require('./compat-table/data-es2016plus.js');
const esnext = require('./compat-table/data-esnext.js');

// TODO: add target for diff file.
const data = [...es5.tests, ...es6.tests, ...es2016Plus.tests, ...esnext.tests].map((test) => {
  return mapper(test)
});

console.log(JSON.stringify(data, null, 2));

function mapper(test) {
  return {
    name: test.name,
    category: test.category,
    significance: test.significance,
    spec: test.spec,
    mdn: test.mdn,
    exec: test.exec && testScript(test.exec).trim(),
    subtests: test.subtests && test.subtests.map(mapper)
  };
}

function testScript(fn) {
  function deindentFunc(fn) {
    fn = (fn+'');
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
    var expr = (fn+"").match(/[^]*\/\*([^]*)\*\/\}$/);
    // if there wasn't an expression, make the function statement into one
    if (!expr) {
        expr = deindentFunc(fn);
      return expr;
    } else {
      expr = deindentFunc(expr[1]);
      return expr
    }
  } else {
    // it's an array of objects like the following:
    // { type: 'application/javascript;version=1.8', script: function () { ... } }
    return fn.reduce(function(text, script) {
      var expr = deindentFunc(
          (script.script+'').replace(/^function \(\) \{\s*|\s*\}$/g, '')
        );
      return text + '\n' + expr;
    },'');
  }
}

