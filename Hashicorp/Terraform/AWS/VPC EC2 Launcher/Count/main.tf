provider aws {
    region = var.aws_region
    shared_credentials_file = var.shared_credentials_path
    profile = "default"
}

/*Cria a chave publica e a privada no diretorio onde o Terraform está rodando.*/
resource "null_resource" "public_private_key" {
    provisioner "local-exec" {
        command = "ssh-keygen -f ${path.module}/${var.prefix_name}_key_pair_launcher -m pem -N ''"
        } 
    /*Remove a chave privada quando fizermos terraform destroy*/
    provisioner "local-exec" {
        when = destroy
        command = "rm ./*_key_pair_launcher"
    }
    /*Remove a chave publica quando fizermos terraform destroy*/
    provisioner "local-exec" {
        when = destroy
        command = "rm ./*key_pair_launcher.pub"
    }
    
}
/*Necessario pois não podemos usar a função file para ler a chave, funções não participam
da dependencia do Terraform.*/
data "local_file" "read_key" {
    depends_on = [null_resource.public_private_key]
    filename = "${path.module}/${var.prefix_name}_key_pair_launcher.pub"
}
/*Passando a chave publica que criamos para a AWS.*/
resource "aws_key_pair" "public_key" {
    key_name = "${var.prefix_name}_key_pair_launcher"
    public_key = data.local_file.read_key.content
}

/*Criando o VPC*/
resource "aws_vpc" "vpc" {
    cidr_block = var.vpc_cidr_block
    tags = {
        "Name" = "${var.prefix_name}_vpc"
    }
}
/*Cria uma ou varias subnets dentro do VPC, de acordo com a quantidade dentro da lista vpc_subnets*/
resource "aws_subnet" "vpc_subnet"{
    count = length(var.vpc_subnets)
    cidr_block = var.vpc_subnets[count.index]
    vpc_id = aws_vpc.vpc.id
    map_public_ip_on_launch = "true"
    tags = {
        "Name" = "${var.prefix_name}_${var.vpc_subnets[count.index]}_subnet"
    }
}

/*Cria um internet gateway para termos saida para a internet do VPC*/
resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.vpc.id

  tags = {
    Name = "${var.prefix_name}_igw"
  }
}

/*Criando rota default ipv4 para associar a tabela de roteamento default*/

resource "aws_route" "ipv4_default_route" {
    destination_cidr_block = "0.0.0.0/0"
    route_table_id = aws_vpc.vpc.main_route_table_id
    gateway_id = aws_internet_gateway.igw.id
}

/*Criando rota default ipv6 para associar a tabela de roteamento default*/

resource "aws_route" "ipv6_default_route" {
    destination_ipv6_cidr_block = "::/0"
    route_table_id = aws_vpc.vpc.main_route_table_id
    gateway_id = aws_internet_gateway.igw.id
}

/*Associando as subnets a tabela de roteamento default do VPC*/

resource "aws_route_table_association" "subnets_associantion" {
  count = length(var.vpc_subnets)
  subnet_id      = aws_subnet.vpc_subnet[count.index].id
  route_table_id = aws_vpc.vpc.main_route_table_id
}

/*Criando security group para as instancias*/
resource "aws_security_group" "security_group" {
  name        = "${var.prefix_name}_security_group"
  description = "Created by Terraform"
  vpc_id      = aws_vpc.vpc.id

  tags = {
    Name = "${var.prefix_name}_security_group"
  }
}

resource "aws_security_group_rule" "sg_ingress_rule" {
  count = length(var.security_group_ingress_ports)
  type              = "ingress"
  from_port         = var.security_group_ingress_ports[count.index]
  to_port           = var.security_group_ingress_ports[count.index]
  protocol          = var.security_group_ingress_protocol[count.index]
  cidr_blocks       = var.security_group_ingress_cidr
  security_group_id = aws_security_group.security_group.id
}

resource "aws_security_group_rule" "sg_egress_rule" {
  count = length(var.security_group_egress_ports)
  type              = "egress"
  from_port         = var.security_group_egress_ports[count.index]
  to_port           = var.security_group_egress_ports[count.index]
  protocol          = var.security_group_egress_protocol[count.index]
  cidr_blocks       = var.security_group_egress_cidr
  security_group_id = aws_security_group.security_group.id
}

/*resource "aws_network_acl" "net_acl" {
  vpc_id = aws_vpc.vpc.id
  subnet_ids =
  tags = {
    Name = "${var.prefix_name}_net_acl"
  }
}*/


/*resource "aws_network_acl_rule" "net_ingress_rule" {
  network_acl_id = aws_network_acl.net_acl.id
  rule_number    = 200
  egress         = false
  protocol       = "tcp"
  rule_action    = "allow"
  cidr_block     = aws_vpc.foo.cidr_block
  from_port      = 22
  to_port        = 22
}

resource "aws_network_acl_rule" "net_egress_rule" {
  network_acl_id = aws_network_acl.net_acl.id
  rule_number    = var.net_egress_rule_number
  egress         = true
  protocol       = var.net_egress_protocol
  rule_action    = "allow"
  cidr_block     = aws_vpc.foo.cidr_block
  from_port      = 22
  to_port        = 22
}
*/
output "igw_id" {
  value = aws_internet_gateway.igw.id
  description = "ID do IGW criado na AWS"
}