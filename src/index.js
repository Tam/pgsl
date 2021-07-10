#!/usr/bin/env node

!async function () {
	process.title = 'pgsl';

	switch (process.argv[2]) {
		case 'tokenize':
			console.time();
			const tokenizer = new (await require('./tokenizer'))();
			await tokenizer.load(process.argv[3]);
			console.dir(
				Array.from(tokenizer),
				{ 'maxArrayLength': null }
			);
			console.timeEnd();
			break;
		case 'parse':
			await require('./parser')(await require('./tokenizer')(process.argv[3]));
			break;
		default:
			console.log('TODO: help');
	}
}();
