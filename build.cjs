const es2016 = require('./compat-table/data-es2016plus.js');

const data = es2016.tests.map((test) => {
  return {
    name: test.name,
  };
});

console.log(JSON.stringify(data, null, 2));
