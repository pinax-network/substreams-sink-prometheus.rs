import { Substreams, download } from "substreams";
import { listen } from "./server"

export function run(args: {
    spkg?: string,
    outputModule?: string,
    startBlockNum?: string,
    substreamsEndpoint?: string,
} = {}) {
    // User params
    const spkg = "https://github.com/pinax-network/substreams/releases/download/eosmechanics-v0.2.0/eosmechanics-v0.2.0.spkg";
    const messageTypeName = "pinax.substreams.sink.prometheus.v1.PrometheusOperations";
    const outputModule = args.outputModule ?? "prom_out";
    const startBlockNum = args.startBlockNum ?? "0";
    const host = args.substreamsEndpoint ?? "mainnet.eth.streamingfast.io:443";

    console.log(args)
    
    // Initialize Substreams
    const substreams = new Substreams(outputModule, {
        host,
        startBlockNum,
        authorization: process.env.STREAMINGFAST_KEY // or SUBSTREAMS_API_TOKEN
    });

    // Prometheus Exporter
    listen(9102).then(async () => {
        // download Substream from IPFS
        const { modules, registry } = await download(spkg);
    
        // Find Protobuf message types from registry
        const PrometheusOperations = registry.findMessage(messageTypeName);
        if (!PrometheusOperations) throw new Error(`Could not find [${messageTypeName}] message type`);
        
        substreams.on("mapOutput", output => {
            if (output.name !== outputModule) return;
            const decoded = PrometheusOperations.fromBinary(output.data.mapOutput.value);
    
            // Prometheus metrics
            for ( const decodedOperation of decoded.operations ) {
                const { metric, operation, name, value, labels } = decodedOperation;
                console.log(decodedOperation);
                // TODO: send to Prometheus
            }
        });
    
        // start streaming Substream
        await substreams.start(modules);
    });
}
