process.on('SIGINT', () => {
    console.log('exiting...');
    process.exit(0);
});

console.log(`starting [${process.pid}]`);
setTimeout(() => {}, 60*60*60*24);
