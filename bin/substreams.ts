import { Substreams, download } from "substreams";
import { handleOperation } from "./metrics";
import { listen } from "./server"

export async function run(spkg: string, args: {
    outputModule?: string,
    startBlock?: string,
    substreamsEndpoint?: string,
    port?: string,
    address?: string,
} = {}) {
    // User params
    const messageTypeName = "pinax.substreams.sink.prometheus.v1.PrometheusOperations";
    const outputModule = args.outputModule;
    const startBlockNum = args.startBlock;
    const host = args.substreamsEndpoint;
    const port = Number(args.port);
    const address = args.address;
    if ( !outputModule ) throw new Error("Missing outputModule");
    
    // Initialize Substreams
    const substreams = new Substreams(outputModule, {
        host,
        startBlockNum,
        authorization: process.env.STREAMINGFAST_KEY // or SUBSTREAMS_API_TOKEN
    });

    // Initialize Prometheus server
    listen(port, address);

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
