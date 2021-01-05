variable "shared_credentials_path" {}
variable "aws_region" {}
variable "prefix_name"{
    type = string
    description = "Digite o nome do prefixo, todos os recursos terão esse prefixo:"
}
variable "vpc_cidr_block"{
}
variable "vpc_subnets" {
}