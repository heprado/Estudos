variable "provider_config" {
    type = map
    default = {
    "region" = "us-east-1"
    "credentials_path" = "./credentials"
    "profile" = "default"
    }
}

variable prefix_name {
    /*Prefixo que será usado para criação de objetos na AWS*/
    default = "HAWS_"
}

variable vpc_cidr_block {
    /*CIDR que será usada para criação de subnets dentro do VPC*/
    default = "192.168.0.0/16"
}

variable subnets {
    /*Subnets que serão criadas dentro do VPC*/
    type = list
    default = ["192.168.1.0/24","192.168.2.0/24"]
}


