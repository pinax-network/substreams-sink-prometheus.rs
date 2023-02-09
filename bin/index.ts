import { Substreams, download } from "substreams";
import { listen, register } from "./server"
import { Counter } from "prom-client";

export function run(args: {
    spkg?: string,
    outputModule?: string,
    startBlockNum?: string,
    substreamsEndpoint?: string,
} = {}) {
    // User params
    const spkg = "https://github.com/pinax-network/substreams/releases/download/eosmechanics-v0.3.2/eosmechanics-v0.3.2.spkg";
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

    // Prometheus Exporter
    listen(9102).then(async () => {
        // download Substream from IPFS
        const { modules, registry } = await download(spkg);
        const metrics = new Map<string, Counter<any>>();
    
        // Find Protobuf message types from registry
        const PrometheusOperations = registry.findMessage(messageTypeName);
        if (!PrometheusOperations) throw new Error(`Could not find [${messageTypeName}] message type`);
        
        substreams.on("mapOutput", output => {
            if (!output.data.mapOutput.typeUrl.match(messageTypeName)) return;
            const decoded = PrometheusOperations.fromBinary(output.data.mapOutput.value);
    
            // Prometheus metrics
            for ( const decodedOperation of decoded.operations ) {
                const { metric, operation, name, value, labels } = decodedOperation.toJson();
                if ( !name ) continue;

                // Only include COUNTER for now
                if ( metric != "METRICS_COUNTER") continue;

                // Register metric
                if ( !metrics.has(name) ) {
                    const counter = new Counter({name, help: "help"});
                    metrics.set(name, counter);
                    register.registerMetric(counter);
                }

                if ( operation == "OPERATIONS_INC" ) {
                    console.log({name, operation, value})
                    metrics.get(name)?.inc(value);
                }
            }
        });
    
        // start streaming Substream
        await substreams.start(modules);
    });
}
