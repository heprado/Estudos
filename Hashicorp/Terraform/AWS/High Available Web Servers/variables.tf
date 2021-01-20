variable "provider_config" {
    type = map
    default = {
    "region" = "us-east-1"
    "credentials_path" = "./credentials"
    "profile" = "default"
    }
}

variable prefix_name {
    default = "HAWS_"
}

variable vpc_cidr_block {
    default = "192.168.0.0/16"
}