import { Command } from "commander";
import { run } from ".";

// CLI user params
const spkg = "" // TODO: add your own spkg via CLI
const outputModule = "" // TODO: add your own output module via CLI
const startBlockNum = "" // TODO: add your own start block number via CLI
const host = "" // TODO: add your own host via CLI
const version = "" // TODO: pull from package.json

const program = new Command();

program.name('substreams-sink-prometheus')
    .version(version, '-v, --version', 'version for substreams-sink-prometheus')
    
    program.command('run')
    .description('Fills Prometheus metrics from a substreams output and runs a Prometheus Exporter listener')
    .option('--delay-before-start <duration>', '[OPERATOR] Amount of time in milliseconds (ms) to wait before starting any internal processes, can be used to perform to maintenance on the pod before actually letting it starts', '0')
    .option('-e --substreams-endpoint <host>', 'Substreams gRPC endpoint', 'mainnet.eth.streamingfast.io:443')
    .action(run);

program.command('completion').description('Generate the autocompletion script for the specified shell');

program.command('help').description('Help about any command');

program.parse();
