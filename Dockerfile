FROM grafana/grafana:8.1.7

COPY ./datasources.yaml /etc/grafana/provisioning/datasources/datasources.yaml
COPY ./dashboardProviders.yaml /etc/grafana/provisioning/dashboards/dashboardProviders.yaml
COPY ./dashboards /var/lib/grafana/dashboards