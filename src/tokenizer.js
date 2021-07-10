const fs = require('fs')
	, path = require('path')
	, readline = require('readline');

module.exports = async function tokenizer (fileName) {
	// Open the file stream
	const fileStream = fs.createReadStream(path.join(
		process.cwd(), // Must be relative to our current dir
		fileName.replace(/\.pgl$/, '') + '.pgl' // make extension optional
	));

	// Read it line by line
	const rl = readline.createInterface({
		input: fileStream,
		crlfDelay: Infinity, // Support any type of linebreak combination
	});

	const tokens = [];

	// Loop over each line
	for await (const line of rl) {
		let isIndenting = true,
			word = '';

		// Push the current word
		const pushWord = () => {
			if (word === '') return;

			tokens.push({
				type: 'WORD',
				value: word,
			});

			word = '';
		};

		// Loop over each character
		for (let i = 0, l = line.length; i < l; i++) {
			const char = line.charAt(i);

			// When we see a space or tab
			if (/[ \t]/.test(char)) {

				// If we're indenting, count it as part of an indent
				if (isIndenting) {
					// If we've already got an indent token, increase it's depth
					if (tokens[tokens.length - 1].type === 'INDENT')
						tokens[tokens.length - 1].value++;

					// Otherwise push a new indent token
					else tokens.push({ type: 'INDENT', value: 1 });
				}

				// If we're not indenting push the whitespace
				else {
					// Push the current word if we have one
					pushWord();

					// If we've already got a whitespace token, append the char
					if (tokens[tokens.length - 1].type === 'WHITESPACE')
						tokens[tokens.length - 1].value += char;

					// Otherwise push a new whitespace token (if it's a tab or
					// part of a sequence of spaces)
					else if (char === '\t' || line.charAt(i + 1) === ' ')
						tokens.push({ type: 'WHITESPACE', value: char });
				}
			}

			// If the char is a # it's a comment, skip the rest of the line
			else if (char === '#') break;

			// If it's any other character
			else {
				// We're no longer indenting
				isIndenting = false;

				// If it's a word character, add it to the current word
				if (/\w/.test(char)) word += char;

				// If it's any other character, it's an operator
				else {
					// Push the current word if we have one
					pushWord();

					// Push the operator
					tokens.push({
						type: 'OPERATOR',
						value: char,
					});
				}
			}
		}

		// Push the remaining word
		pushWord();

		// Skip continuous EOLs and EOLs at the start of the document
		if (tokens.length === 0 || tokens[tokens.length - 1].type === 'EOL')
			continue; // to next line

		// Push the EOL
		tokens.push({ type: 'EOL' });
	}

	// Drop the last EOL
	tokens.pop();

	// Return the tokens
	return tokens;
};
