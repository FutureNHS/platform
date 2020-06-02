# 2. Observability tools and strategy

Date: 2020-05-26

## Status

Accepted

## Context

We need to be able to monitor our cluster / apps, and be notified by automated alerts if problems occur.

## Decision
We will base our solution around Azure Monitor and its related services. As a managed service, it means:
 - Scaling, updates and data storage are taken care of as part of the solution, resulting in lower running costs.
 - State remains outside the cluster.  Together with GitOps, which stores Kubernetes resource manifests in an external git repository, this makes it's easy to recreate the cluster at any point in time.

### Metrics
We will use [Prometheus](https://prometheus.io/) to gather metrics for the following reasons:
 - Both [Linkerd](https://linkerd.io/2/tasks/exporting-metrics/) and [Istio](https://istio.io/docs/tasks/observability/metrics/querying-metrics/) (our potential service mesh choices) have an internal Prometheus instance.
 - Proven and popular, wide compatibility, Cloud Native Computing Foundation project.
 - We can leverage some of the knowledge and existing work done by the [MHRA team](https://github.com/MHRA/products).
 - Close integration with Grafana should we decide to use it (pre-existing dashboard templates for exporters etc).

Prometheus support within [Azure Monitor for Containers](https://docs.microsoft.com/en-us/azure/azure-monitor/insights/container-insights-overview) means that no separate Prometheus server is required but allows for scraping `/metrics` endpoints and making use of Prometheus queries for custom alerts etc.

An example of Prometheus Query (PromQL) can be found here. https://github.com/MHRA/deployments/blob/e42c0a9ee320294c1d691392c5b8525703cdd524/observability/prometheus-configmap.yaml#L15


### Tracing
Instrumentation for our apps will follow the [OpenTelemetry](https://opentelemetry.io/) API.

Will be consumed by [Azure Application Insights](https://docs.microsoft.com/en-us/azure/azure-monitor/app/distributed-tracing) or [Jaeger](https://www.jaegertracing.io/). A spike will happen at a later date to determine what we require here.


### Logging
Azure Monitor pulls in logs via its Metrics API, meaning:
 - We can use Log Analytics in the Azure portal to write log queries and interactively analyze log data.
 - We can use the Application Insights analytics console in the Azure portal to write log queries and interactively analyze log data from Application Insights.

### Alerting
Alerts will be configured through Azure Monitor.

Azure uses the Kusto query language. An example can be found [here] (https://github.com/MHRA/products/blob/d609203a5126bdb1ab9bcf1e5d9f910b49231ca1/infrastructure/modules/cluster/alerts.tf#L29).

Action Groups can be associated with alerts to handle SMS / email notifications for people on support duty.

Note: Prometheus also has an ‘Alert Manager’ which can handle some / much of the above. We should be able to compare the two fairly easily once up and running. Secondly, there may be additional value in combining the tech issues with the existing user support system. A spike to investigate this has been added to the backlog. 

### Visualisation
Azure Dashboard offers basic functionality and may be sufficient, however Grafana may well be worth setting up due to the availability of pre-existing templates for a number of our needs (eg node exporter dashboard), as well as its greater flexibility. Spike to follow.


## Consequences

No particular risks at this stage to mitigate from our above choices - obviously we are still at a very early stage and have ample room to manoeuvre.  Pricing for Azure monitoring can be found [here](https://azure.microsoft.com/en-gb/pricing/details/monitor/).
