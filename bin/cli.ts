import { Substreams, download } from "substreams";
import { Command } from "commander";

import { listen } from "./server"

// Substreams using live data
const spkg = "https://github.com/pinax-network/substreams/releases/download/eosmechanics-v0.2.0/eosmechanics-v0.2.0.spkg";
const outputModule = "prom_out";
const startBlockNum = "288786307";
const host = "eos.firehose.eosnation.io:9001"

// Initialize Substreams
const substreams = new Substreams(outputModule, {
    host,
    startBlockNum,
    authorization: process.env.STREAMINGFAST_KEY // or SUBSTREAMS_API_TOKEN
});

const program = new Command();

program.name('substreams-sink-prometheus')
    .version('0.1.1', '-v, --version', 'version for substreams-sink-prometheus')
    .option('--delay-before-start <duration>', '[OPERATOR] Amount of time in milliseconds (ms) to wait before starting any internal processes, can be used to perform to maintenance on the pod before actually letting it starts', '0');

program.command('run')
    .description('Fills Prometheus metrics from a substreams output and runs a Prometheus Exporter listener')
    .action(async () => {

        let options = program.opts();

        // Prometheus Exporter
        listen(9102).then(async () => {
            // download Substream from IPFS
            const { modules, registry } = await download(spkg);

            // Find Protobuf message types from registry
            const PrometheusOperations = registry.findMessage("pinax.substreams.sink.prometheus.v1.PrometheusOperations");
            if (!PrometheusOperations) throw new Error("Could not find PrometheusOperations message type");

            let block_num = 0;
            substreams.on("block", block => {
                block_num = Number(block?.clock?.number);
            });

            substreams.on("mapOutput", output => {
                if (output.name !== "prom_out") return;
                const decoded = PrometheusOperations.fromBinary(output.data.mapOutput.value);

                // Prometheus metrics
                for (const { labels, value, type } of decoded.operations) {
                    for (const item of labels) {
                        const [gauge, label] = item.split(":");
                        console.log(block_num, JSON.parse(JSON.stringify({ gauge, label, value, type })));
                        // // SET
                        // if (type === 1) {
                        //     if ( label ) gauges[gauge].labels(label).set(value);
                        //     else gauges[gauge].set(value);
                        // }
                        // // RESET
                        // if (type === 2) gauges[gauge].reset(label);
                        // // INC
                        // if (type === 3) gauges[gauge].inc(label);
                    }
                }
            });

            // start streaming Substream
            await substreams.start(modules);
        });
    });

program.command('completion').description('Generate the autocompletion script for the specified shell');

program.command('help').description('Help about any command');

program.parse();
