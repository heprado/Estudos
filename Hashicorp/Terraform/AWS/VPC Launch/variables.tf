variable "region" {
  type = string
  description = "ID da Região que vamos subir o VPC e a EC2"
}

variable "prefix_name" {
  type = string
  description = "Prefixo que sera utilizado no nomes do objetos"
}

variable "vpc_cidr" {
    type = string
    description = "Range do VPC que será criado. CIDR ENTRE /16 e /28  Exemplo (10.0.0.0/16)"
}

variable "subnet" {
    type = string
    description = "Subnet que será criada. Exemplo - 10.10.10.0/24"
}