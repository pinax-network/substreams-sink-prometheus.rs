import { Counter, Gauge } from "prom-client";
import { register } from "./server";

export function handleOperation(promOp: PrometheusOperation<any>) {
    handleGauge(promOp);
    handleCounter(promOp)
}

function handleCounter(promOp: PrometheusOperation<CounterOp>) {
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
    }
}

function handleGauge(promOp: PrometheusOperation<GaugeOp>) {
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
    }
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

interface PrometheusOperation<T = any> {
    name: string
    labels: {[key: string]: string},
    operation: T
}

interface GaugeOp {
    case: "gauge",
    value: { operation: number, value: number }
}

interface CounterOp {
    case: "counter",
    value: { operation: number, value: number }
}
