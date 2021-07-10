#!/usr/bin/env node

!async function () {
	process.title = 'pgsl';

	switch (process.argv[2]) {
		case 'tokenize':
			console.log(await require('./tokenizer')(process.argv[3]));
			break;
		default:
			console.log('TODO: help');
	}
}();
