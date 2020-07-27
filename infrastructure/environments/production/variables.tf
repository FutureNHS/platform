variable "location" {
  description = "Azure region"
  default     = "westeurope"
}

variable "ip_whitelist_insights" {
  description = "List of whitelisted IPs for use with Synapse"
  type        = map(string)
}
