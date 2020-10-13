"use strict";

const {
  AzureMonitorTraceExporter,
} = require("@azure/monitor-opentelemetry-exporter");
const { NodeTracerProvider } = require("@opentelemetry/node");
const {
  BatchSpanProcessor,
  ConsoleSpanExporter,
} = require("@opentelemetry/tracing");

const provider = new NodeTracerProvider({
  plugins: {
    https: {
      // Ignore Application Insights Ingestion Server
      ignoreOutgoingUrls: [new RegExp(/dc.services.visualstudio.com/i)],
    },
  },
});

const exporter =
  process.env.NEXT_PUBLIC_INSTRUMENTATION_KEY && 0
    ? new AzureMonitorTraceExporter({
        logger: provider.logger,
        instrumentationKey: process.env.NEXT_PUBLIC_INSTRUMENTATION_KEY,
      })
    : new ConsoleSpanExporter();

provider.register();

provider.addSpanProcessor(
  new BatchSpanProcessor(exporter, {
    bufferTimeout: 1000,
    bufferSize: 1000,
  })
);

module.exports = { provider };
