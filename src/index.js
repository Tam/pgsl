#!/usr/bin/env node

!async function () {
	process.title = 'pgsl';

	switch (process.argv[2]) {
		case 'tokenize':
			console.time();
			console.dir(
				await require('./tokenizer')(process.argv[3]),
				{ 'maxArrayLength': null }
			);
			console.timeEnd();
			break;
		default:
			console.log('TODO: help');
	}
}();
