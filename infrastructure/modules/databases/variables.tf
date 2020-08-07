variable "environment" {
  description = "Namespace for all resources. Eg 'production' 'dev-jane'"
}

variable "postgresql_server_name" {
  description = "Name of the PostgreSQL server, where databases and users should be created"
}

variable "eventgrid_topic_endpoint" {
  description = "Endpoint of the platform Event Grid Topic"
}

variable "eventgrid_topic_key" {
  description = "Access key of the platform Event Grid Topic"
}
