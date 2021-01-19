/*Caminho para o arquivo com as credenciais do IAM que vão ser utilizadas para criar o VPC e as EC2
    
    Abaixo segue como o arquivo deve ser formatado:

    [default]
    aws_access_key_id = "seu access key id"
    aws_secret_access_key = "sua secret access key"
*/

shared_credentials_path = "/home/tidebinder/Github/Studies/Hashicorp/Terraform/AWS/VPC EC2 Launcher/credentials"

/*Região que vamos criar os recursos na AWS*/

aws_region = "us-east-1"

/*Todos os recursos vão ser criados com esse prefixo para fácil idenficação na AWS.*/

prefix_name = "heprado"

/*CIDR que será utilizada para o VPC.*/

vpc_cidr_block = "172.16.0.0/16"

/*Subnets que vão ser criados dentro do VPC, colocar uma LISTA DE SUBNETS mesmo se for somente uma.*/

vpc_subnets = ["172.16.1.0/24", "172.16.2.0/24"]

/*Portas que serão liberadas para tráfego Ingress dentro do Security Group*/

security_group_ingress_ports = [22,443,0]

/*Valores possiveis são "tcp","udp","icmp","-1"(Todos os protocolos), os valores precisam estar na mesma ordem da porta que você quer liberar
na variavel security_group_ingress_ports*/

security_group_ingress_protocol = ["tcp","tcp","icmp"]

/*Lista de CIDRs que esse tráfego Ingress sera permitido dentro do Security Group*/

security_group_ingress_cidr = ["0.0.0.0/0"]


/*Portas que serão liberadas para tráfego egress dentro do Security Group*/

security_group_egress_ports = [0]

/*Lista de CIDRs que esse tráfego egress sera permitido dentro do Security Group*/

security_group_egress_cidr = ["0.0.0.0/0"]

/*Valores possiveis são "tcp","udp","icmp","-1"(Todos os protocolos), os valores precisam estar na mesma ordem da porta que você quer liberar
na variavel security_group_egress_ports*/

security_group_egress_protocol = ["-1"]



