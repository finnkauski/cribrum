import('./pkg').then(
    m => console.log(m.tokenize("From within JS"))
).catch(console.error);
