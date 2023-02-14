import { Substreams, download } from "substreams";
import { handleOperation } from "./metrics";
import { listen } from "./server"

export async function run(args: {
    spkg?: string,
    outputModule?: string,
    startBlockNum?: string,
    substreamsEndpoint?: string,
} = {}) {
    // User params
    const spkg = "https://github.com/pinax-network/substreams/releases/download/eosmechanics-v0.3.3/eosmechanics-v0.3.3.spkg";
    const messageTypeName = "pinax.substreams.sink.prometheus.v1.PrometheusOperations";
    const outputModule = args.outputModule ?? "prom_out";
    const startBlockNum = args.startBlockNum ?? "292442484";
    const host = args.substreamsEndpoint ?? "mainnet.eth.streamingfast.io:443";
    
    // Initialize Substreams
    const substreams = new Substreams(outputModule, {
        host,
        startBlockNum,
        authorization: process.env.STREAMINGFAST_KEY // or SUBSTREAMS_API_TOKEN
    });

    // Initialize Prometheus server
    listen(9102);

    // Download Substream from URL or IPFS
    const { modules, registry } = await download(spkg);

    // Find Protobuf message types from registry
    const PrometheusOperations = registry.findMessage(messageTypeName);
    if (!PrometheusOperations) throw new Error(`Could not find [${messageTypeName}] message type`);
    
    substreams.on("mapOutput", output => {
        // Handle Prometheus Operations
        if (!output.data.mapOutput.typeUrl.match(messageTypeName)) return;
        const decoded = PrometheusOperations.fromBinary(output.data.mapOutput.value);
        for ( const operation of decoded.operations ) {
            handleOperation(operation);
        }
    });

    // start streaming Substream
    await substreams.start(modules);
}
