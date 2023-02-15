import client from "prom-client";
import http from "node:http";

// Prometheus Exporter
export const register = new client.Registry();
const collectDefaultMetrics = client.collectDefaultMetrics;
collectDefaultMetrics({ register });

// Create a local server to serve Prometheus gauges
const server = http.createServer(async (req, res) => {
    res.writeHead(200, { 'Content-Type': register.contentType });
    res.end(await register.metrics());
});

export async function listen(port: number, address = "0.0.0.0") {
    return new Promise(resolve => {
        server.listen(port, address, () => {
            console.log('starting prometheus metrics server', { address, port });
            resolve(true);
        });
    })
}