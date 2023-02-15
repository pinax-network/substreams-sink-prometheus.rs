import { Command } from "commander";
import { run } from "./substreams";
import pkg from "../package.json";

const program = new Command();
program.name('substreams-sink-prometheus')
    .version(pkg.version, '-v, --version', 'version for substreams-sink-prometheus')
    
program.command('run')
    .description('Fills Prometheus metrics from a substreams output and runs a Prometheus Exporter listener')
    .argument('<spkg>', 'URL or IPFS hash of Substreams package')
    .option('--delay-before-start <int>', '[OPERATOR] Amount of time in milliseconds (ms) to wait before starting any internal processes, can be used to perform to maintenance on the pod before actually letting it starts', '0')
    .option('--delay-before-start <int>', '[OPERATOR] Amount of time in milliseconds (ms) to wait before starting any internal processes, can be used to perform to maintenance on the pod before actually letting it starts', '0')
    .option('-m --output-module <string>', 'Name of the output module (declared in the manifest)', "prom_out")
    .option('-e --substreams-endpoint <string>', 'Substreams gRPC endpoint', 'mainnet.eth.streamingfast.io:443')
    .option('-s --start-block <int>', 'Start block to stream from. Defaults to -1, which means the initialBlock of the first module you are streaming (default -1)')
    .option('-s --end-block <string>', 'Stop block to end stream at, inclusively. (default "0")')
    .option('-p --port <int>', 'Listens on port number.', '9102')
    .option('-a --address <string>', 'Address to use', '0.0.0.0')
    .action(run);

program.command('completion').description('Generate the autocompletion script for the specified shell');

program.command('help').description('Display help for command');

program.parse();
