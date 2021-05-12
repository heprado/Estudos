variable project {
    description = "This is a Tag that will be assigned to everything that this module create."
    default = "HAWS"
}

variable enviroment {
    description = "This is a Tag that will be assigned to everything that this module create."
    default = "DEV"
}

variable prefix {
    description = "This is a prefix that will be assigned before the name of everything created with this module."
    default = "HAWS"
}

variable aws_region {
    description = "The AWS region that this module will deploy the infrastructure"
}

variable "region1_vpc_cidr" {
    description = "The CIDR that will be used in your VPC on Region 1"
    default = "10.10.0.0/16"
}

variable "region1_subnet1_block" {
    description = "The CIDR block that will be assigned to the first subnet"
    default = "10.10.1.0/24"
}

variable "region1_subnet2_block" {
    description = "The CIDR block that will be assigned to the second subnet"
    default = "10.10.2.0/24"
}