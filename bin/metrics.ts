import { Counter, Gauge, Histogram, Summary } from "prom-client";
import { Clock } from "substreams";
import { register } from "./server";
import { logger } from "./logger";

export function handleOperation(promOp: PrometheusOperation) {
    handleGauge(promOp);
    handleCounter(promOp);
    handleSummary(promOp);
    handleHistogram(promOp);
}

export function handleClock(clock: Clock) {
    registerGauge("head_block_number", "Last block number processed by Substreams Sink");
    registerGauge("head_block_timestamp", "Last block timestamp (in seconds) processed by Substreams Sink");
    registerGauge("head_block_drift", "Head block drift (in seconds) by Substreams Sink");
    const gauge1 = register.getSingleMetric("head_block_number") as Gauge;
    const gauge2 = register.getSingleMetric("head_block_timestamp") as Gauge;
    const gauge3 = register.getSingleMetric("head_block_time_drift") as Gauge;
    gauge1.set(Number(clock.number));
    gauge2.set(Number(clock.timestamp?.seconds));
    const head_block_time_drift = Math.floor(new Date().getTime() / 1000) - Number(clock.timestamp?.seconds);
    gauge3.set(head_block_time_drift > 0 ? head_block_time_drift : 0);
}

function handleCounter(promOp: PrometheusOperation) {
    if ( promOp.operation.case != "counter") return;
    const { name, labels } = promOp;
    registerCounter(name, "custom help", Object.keys(labels)); // TO-DO!
    const { operation, value } = promOp.operation.value;
    const counter = register.getSingleMetric(promOp.name) as Counter;
    if ( labels ) counter.labels(labels);
    switch (operation) {
        case 1: counter.labels(labels).inc(); break; // INC
        case 2: counter.labels(labels).inc(value); break; // ADD
        case 7: counter.remove(labels); break; // REMOVE
        case 8: counter.reset(); break; // RESET
        default: return; // SKIP
    }
    logger.log("info", "counter", {name, labels, operation, value});
}

function handleGauge(promOp: PrometheusOperation) {
    if ( promOp.operation.case != "gauge") return;
    const { name, labels } = promOp;
    registerGauge(name, "custom help", Object.keys(labels)); // TO-DO!
    const { operation, value } = promOp.operation.value;
    let gauge = register.getSingleMetric(promOp.name) as Gauge;
    switch (operation) {
        case 1: gauge.labels(labels).inc(); break; // INC
        case 2: gauge.labels(labels).inc(value); break; // ADD
        case 3: gauge.labels(labels).set(value); break; // SET
        case 4: gauge.labels(labels).dec(); break; // DEC
        case 5: gauge.labels(labels).dec(value); break; // SUB
        case 6: gauge.labels(labels).setToCurrentTime(); break; // SET_TO_CURRENT_TIME
        case 7: gauge.remove(labels); break; // REMOVE
        case 8: gauge.reset(); break; // RESET
        default: return; // SKIP
    }
    logger.log("info", "gauge", {name, labels, operation, value});
}

function handleSummary(promOp: PrometheusOperation) {
    if ( promOp.operation.case != "summary") return;
    const { name, labels } = promOp;
    registerSummary(name, "custom help", Object.keys(labels)); // TO-DO!
    const { operation, value } = promOp.operation.value;
    let summary = register.getSingleMetric(promOp.name) as Summary;
    switch (operation) {
        case 1: summary.labels(labels).observe(value); break; // OBSERVE
        case 2: summary.labels(labels).startTimer(); break; // START_TIMER
        case 7: summary.remove(labels); break; // REMOVE
        case 8: summary.reset(); break; // RESET
        default: return; // SKIP
    }
    logger.log("info", "summary", {name, labels, operation, value});
}

function handleHistogram(promOp: PrometheusOperation) {
    if ( promOp.operation.case != "histogram") return;
    const { name, labels } = promOp;
    registerHistogram(name, "custom help", Object.keys(labels)); // TO-DO!
    const { operation, value } = promOp.operation.value;
    let histogram = register.getSingleMetric(promOp.name) as Histogram;
    switch (operation) {
        case 1: histogram.labels(labels).observe(value); break; // OBSERVE
        case 2: histogram.labels(labels).startTimer(); break; // START_TIMER
        case 3: histogram.zero(labels); break; // ZERO
        case 7: histogram.remove(labels); break; // REMOVE
        case 8: histogram.reset(); break; // RESET
        default: return; // SKIP
    }
    logger.log("info", "histogram", {name, labels, operation, value});
}

function registerCounter(name: string, help = "help", labelNames: string[] = []) {
    try {
        register.registerMetric(new Counter({name, help, labelNames}));
    } catch (e) {
        //
    }
}

function registerGauge(name: string, help = "help", labelNames: string[] = []) {
    try {
        register.registerMetric(new Gauge({name, help, labelNames}));
    } catch (e) {
        //
    }
}

function registerHistogram(name: string, help = "help", labelNames: string[] = []) {
    // TO-DO extract from substreams.yaml as config
    const buckets = [0.001, 0.01, 0.1, 1, 2, 5];
    try {
        register.registerMetric(new Histogram({name, help, labelNames}));
    } catch (e) {
        //
    }
}

function registerSummary(name: string, help = "help", labelNames: string[] = []) {
    // TO-DO extract from substreams.yaml as config
    const percentiles = [0.01, 0.1, 0.9, 0.99];
	const maxAgeSeconds: number = 600;
	const ageBuckets: number = 5;
	const compressCount: number = 1;
    try {
        register.registerMetric(new Summary({name, help, labelNames}));
    } catch (e) {
        //
    }
}

interface PrometheusOperation {
    name: string
    labels: {[key: string]: string},
    operation: {
        case: "gauge" | "counter" | "summary" | "histogram",
        value: { operation: number, value: number }
    }
}
