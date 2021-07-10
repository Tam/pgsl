module.exports = async function parser (tokens) {
	const indent = { depth: 0, index: 0 };

	for (let token of tokens) {
		console.log(token);
	}
};
